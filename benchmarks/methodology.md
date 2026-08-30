# Benchmark Methodology

Reproducibility contract for `benchmarks/results/`. Pins down *what* we measure, *how* we measure it, and *what every result file carries* so any contributor on similar hardware can re-run a row of the cross-language tables and land within single-digit percent of the published median.

It also frames Phase 3 implementation: every decision below has a corresponding scaffolding obligation listed in the [Status checklist](#status-checklist).

See [`README.md`](README.md) for current results and [`../ROADMAP.md`](../ROADMAP.md) for the broader v2.6.0 plan.

## Goals

Phase 3 must produce defensible numbers for three questions the Phase 1 [indicative timings](README.md#indicative-timings) and Phase 2 [cost per heap comparison](README.md#cost-per-heap-comparison-huge_dense-derived-from---stats) can only gesture at:

1. **Cross-language wall-time spread** — same algorithm, same operation counts (proven byte-for-byte identical via `--stats`); how much do the five languages actually differ once we warm and summarize statistically?
2. **Arity sweet spot per density bucket** — d=2 vs d=4 vs d=8 across sparse / dense / grid.
3. **Memory cost** — peak RSS on `huge_dense` per (language, arity).

Explicitly **out of scope** for Phase 3:
- Per-section timing *inside* `dijkstra` (hash-map vs heap-op vs comparator). Phase 2 cost-per-comparison strongly suggests Rust's `String` clones + SipHash and Zig's `std.StringHashMap` dominate. — **Moved into scope as Phase 3.5; see § below.**
- Bootstrap CIs, Mann-Whitney, regression detection (Criterion-style).
- Comparisons against `std::priority_queue` / `BinaryHeap` baselines.
- Multi-threaded variants.

## Measurement contract

### Signals

| Signal | Source | Cardinality |
|--------|--------|-------------|
| Wall time | `dijkstra(graph, source)` function call only | per (lang, graph, arity) |
| Comparison counts | `--stats` output, separate process invocation | per (lang, graph, arity) |
| Peak RSS | OS process accounting after one un-iterated `dijkstra` call | per (lang, arity) on `huge_dense` only |

**Wall time excludes graph load and JSON parse.** The `dijkstra` invocation is the unit under test. Parse cost differs across languages for reasons unrelated to the heap (Zig's arena loader, C++'s hand-rolled parser, Rust/Go/TS `serde`/`encoding-json`/`JSON.parse`); folding it in would muddy the heap-cost story Phase 2 surfaced.

**Comparison counts run in a separate process invocation from wall-time runs.** C++/Rust/Zig instrumentation is provably zero-overhead by construction, but Go's nil-pointer check is non-zero (~1 cycle) and TypeScript's JIT elision is empirical. Keeping the runs cleanly separated avoids arguing about it.

**Peak RSS is captured from a single un-iterated invocation.** Looping inflates RSS via allocator high-water marks that have nothing to do with steady-state heap memory.

### Procedure

Per cell: one process invocation runs `dijkstra(graph, source)` **N = 10 times** in a tight loop, printing a JSON record per timed call. The aggregator computes median / IQR / min / max from the 10 timings.

A small warmup discard absorbs JIT and TLB cold-start costs without complicating the harness:

```
warmup = 2     # discarded calls (V8 JIT tier-up, branch predictor, TLB)
N      = 10    # timed calls; median = (5th + 6th) / 2, IQR = Q3 − Q1
```

12 calls total per process, 10 records emitted.

**Reported summary statistics: median, IQR (Q3 − Q1), min, max.** Mean is *not* reported — GC pauses (Go), generational collection (TypeScript), and OS jitter skew the mean upward without telling us anything useful about the steady state. With 10 samples the median is robust to a single high outlier (cold-cache miss, GC pause, or OS preemption); IQR captures the spread; min/max keep the tails visible.

**Known timer-resolution caveat (Go on Windows).** Go's `time.Since` on Windows quantizes around ~500 µs, so the `medium_sparse` Go cell (per-call cost ~150 µs) will snap to a coarse grid and report effectively-quantized medians. Phase 1's table already shows this — `medium_sparse` Go reads as `(sub-res)`. We accept this rather than introduce inner-loop amortization just to fix one cell; the cell is footnoted in the published table, and `medium_dense` / `large_*` / `huge_dense` (where the cross-language story actually lives) all clear the resolution.

### Sample matrix

| Axis | Values | Count |
|------|--------|------:|
| Language | C++, Go, Rust, TypeScript, Zig | 5 |
| Graph | `small`, `medium_sparse`, `medium_dense`, `medium_grid`, `large_sparse`, `large_dense`, `large_grid`, `huge_dense` | 8 |
| Arity | d=2, d=4, d=8 | 3 |
| **Total cells** | | **120** |

`small` is included as a sanity check (degenerate behavior on a 6-vertex graph signals a bug, not a result) but is **excluded from cross-bucket comparison tables** in `README.md` — at ~1 µs per call, the 10 timed calls sit at the edge of timer resolution and are dominated by process / loop overhead, not heap work.

## Build & toolchain (pinned)

| Language | Command | Notes |
|----------|---------|-------|
| C++ | `cmake -B build -DCMAKE_BUILD_TYPE=Release && cmake --build build --config Release` | MSVC `/O2 /utf-8` on Windows; `-O3` on GCC/Clang |
| Go | `go build` | Default optimizer; no `-ldflags` games |
| Rust | `cargo build --release` | Stock `release` profile (LTO off, opt-level=3) |
| TypeScript | `tsc && node dist/dijkstra.js` | **Pre-compiled**, not `tsx`. `tsx` adds per-call transpile cost the JIT cannot fully amortize. |
| Zig | `zig build -Doptimize=ReleaseFast` | Matches C++/Rust optimization levels |

LTO, PGO, and `-march=native` are intentionally **off**. Phase 3 is about cross-language comparison at default optimization, not single-language tuning. A future "tuned" pass is fair game; it does not belong in the same table as the default-optimization numbers.

## Environment reporting

Every JSON record carries an `env` block. Capture once per machine into `benchmarks/scripts/env.json` and inline at write-time so each result file is self-describing:

```json
{
  "os":             "Windows 11 26200.x",
  "cpu":            "13th Gen Intel(R) Core(TM) i7-13700H",
  "cpu_base_ghz":   2.4,
  "cpu_boost_ghz":  5.0,
  "logical_cores":  20,
  "ram_gb":         32,
  "turbo":          "enabled",
  "power_plan":     "High performance",
  "toolchain":      "rustc 1.84.0",
  "build_flags":    "--release",
  "commit":         "0182c7a",
  "date":           "2026-05-10T18:32:11+02:00",
  "host":           "redacted"
}
```

**OS knobs to set before benchmarking** (recommended, not required — we want numbers users actually see, not numbers only the maintainer can reproduce on a tuned rig):

- **Windows**: `powercfg /setactive SCHEME_MIN` (High Performance), close Slack/Teams/browser, pause Windows Update during the run, optionally pin process affinity to performance cores via `Start-Process -ProcessorAffinity`.
- **Linux**: `cpupower frequency-set -g performance`; `taskset -c 2-7` to pin away from core 0; do not disable turbo unless you specifically want stable-not-fast numbers.
- **macOS**: `caffeinate -d` only. Apple Silicon throttles under sustained load — flag macOS results as **indicative**, not tuned.

We do **not** disable turbo by default. Median + IQR over 10 reps absorbs the resulting variance, and the README numbers should reflect realistic conditions.

## Harness

Each language's existing `dijkstra` example binary gains three flags:

```
dijkstra --graph=<name> --arity=<d> --warmup=K --repetitions=N --json
```

Output: **N JSON records on stdout, one per timed `dijkstra` call.** The wrapper script `benchmarks/scripts/<lang>/run.{ps1,sh}` invokes the binary **once per (graph, arity) cell** with `--warmup=2 --repetitions=10` and appends the 10 records to `benchmarks/results/<lang>/<graph>_d<arity>.jsonl`.

**One JSON record = one timed call.** This keeps the runner crash-resilient (a re-run produces a fresh 10-line file rather than half-overwriting an old one) and the schema flat.

A separate stats pass produces `<lang>/<graph>_d<arity>.stats.json` (one record per cell, captured in a clean process with `--stats`).

A separate memory pass produces `<lang>/huge_dense_d<arity>.rss.json` per arity. RSS source: `Get-Process` on Windows, `/usr/bin/time -v` (Max RSS) on Linux, `/usr/bin/time -l` on macOS.

## Output schema

### Wall-time record (`<lang>/<graph>_d<arity>.jsonl`, one line per timed call)

```json
{
  "schema_version": 1,
  "language":       "Rust",
  "graph":          "huge_dense",
  "arity":          8,
  "source":         "v0",
  "target":         "v2499",
  "rep":            6,
  "wall_time_us":   23253.5,
  "env": { /* see above */ }
}
```

The `rep` field (1..N) lets the aggregator detect missing records and lets a reader inspect cold-call inflation if curious. The `env` block is repeated on every record by design — each result file is self-describing, no joins required.

### Comparison-count record (`<lang>/<graph>_d<arity>.stats.json`, single object)

```json
{
  "schema_version": 1,
  "language":       "Rust",
  "graph":          "huge_dense",
  "arity":          8,
  "comparison_counts": {
    "insert":             2499,
    "pop":                63710,
    "increase_priority":  9727,
    "total":              75936
  }
}
```

The `total` field **must agree byte-for-byte across all five languages on a given (graph, arity) cell** — this is the Phase 2 invariant. The aggregator asserts it and exits non-zero on disagreement; a divergence is a bug, not benchmark variance.

### RSS record (`<lang>/huge_dense_d<arity>.rss.json`, single object)

```json
{
  "schema_version": 1,
  "language":       "Rust",
  "graph":          "huge_dense",
  "arity":          8,
  "peak_rss_kb":    18432
}
```

## Aggregation

`benchmarks/scripts/aggregate.{py,rs}` reads `results/**/*.{jsonl,stats.json,rss.json}`, computes per-cell median/IQR/min/max, asserts the comparison-count invariant, and emits:

- `benchmarks/README.md` — four condensed tables (sparse / dense / grid / huge), replacing "Indicative timings."
- `benchmarks/results/SUMMARY.md` — full 120-cell matrix with IQR + min/max.
- `benchmarks/results/cost_per_comparison.md` — refresh of the existing Phase 2 table with stable medians.

Aggregation is deterministic given the same inputs — it must not pick "the latest" run when multiple JSONL records exist for the same cell; it processes everything and groups by `(language, graph, arity)`.

## Reproducibility

The corpus generator (`benchmarks/scripts/graphgen/`) already gives us byte-for-byte deterministic graphs. To make a wall-time result reproducible we additionally need:

- **Frozen source/target** per graph: `A → F` for `small`, `v0 → v(N-1)` for the rest.
- **Frozen build flags** (table above).
- **Toolchain version** in `env.toolchain`.
- **Commit SHA** in `env.commit`.

Anyone with the same toolchain on similar-class hardware should land within single-digit percent of the published medians. Cross-machine variance of 30–50% is normal and uninteresting; cross-machine variance of 3× on a single cell while the other 119 are stable is a bug worth filing.

## Phase 3.5: per-section timing

Phase 3 produced stable median wall times across 5 languages on 24 cells; the 5-way `total` invariant confirms every language performs *exactly* the same number of heap comparisons (75 936 on `huge_dense × d=8`). Yet wall time spreads ~2× — Go 11 016 µs, Rust 22 455 µs. Phase 3.5 attributes that spread: **where, inside `dijkstra(graph, source)`, do the slower languages spend their extra time?**

### Hypothesis to test

Phase 2 ns/cmp ordering (Go 132 → Rust 306) plus the cross-language hash-map analysis suggest:
- **Heap operations** (`pop_front`, `increase_priority`) are roughly uniform across languages — the d-ary heap is the same algorithm everywhere, and the comparator is a single `i32` distance compare.
- **Hashmap traffic** in the relax path (`distances[neighbor]`, `predecessors[neighbor]`, `positions[neighbor]`) is wildly non-uniform: Rust SipHash + `String` clone, V8 `Map<string,…>`, Go flat-bucket AES-NI, Zig Wyhash, MSVC chained-bucket `std::unordered_map` all have very different per-op costs.

If the hypothesis holds, Rust's 11 ms gap vs Go should attribute almost entirely to the relax-path hashmap, not to the heap.

### Sections

Four timed regions per `dijkstra` call:

| Section | Code path | Hypothesis |
|---|---|---|
| `setup` | initial distance/predecessor map init + N `insert` calls into the empty heap | one-shot; ≪ main loop on `huge_dense` |
| `pop` | the `heap.pop_front()` call inside the main loop | heap-only; ~uniform across languages |
| `relax` | adjacency iteration + edge-weight sum + `distances[neighbor]` lookup + comparison + write to `distances` / `predecessors` | hashmap-heavy; expected discriminator |
| `inc_pri` | `heap.increase_priority(...)` when a shorter path is found | heap-only; ~uniform across languages |

Mutually exclusive and (within rounding) exhaustive — every microsecond inside `dijkstra` belongs to one. Cross-language consistency is a contract: each language wraps the *same* lines of code in the same four timer pairs.

### Mechanism

**Revised** (v2.7.0, after the first real run): the original design implied a single un-iterated `dijkstra` call with no warmup and no repetitions, reasoning that "section timers are sub-µs and per-call accumulator variance would dominate at small N." The first real run showed the opposite problem — single-sample noise, not accumulator variance, dominated the signal badly enough that the heap-only `pop` section (expected to be ~uniform across languages) showed a *larger* relative Rust-vs-Go gap than the hashmap-heavy `relax` section (the expected discriminator). Phase 3.5 now reuses Phase 3's own K=2 warmup / N=10 repetitions protocol instead of defending the single-call design:

A `--profile-sections` flag in each example binary:
- Requires `--arity=<d>` (single arity, like `--report-rss`).
- Accepts the same `--warmup=K` / `--repetitions=N` flags the wall-time `--json` path already uses; `benchmarks/scripts/<Lang>/run.ps1` passes `--warmup=2 --repetitions=10`.
- Each language uses its native nano-precision timer (`Instant::now` / `time.Now` / `performance.now` / `std.Io.Clock.Timestamp` / `chrono::high_resolution_clock`).
- Output: one JSON line per timed repetition (K discarded, N printed), each with the four absolute section timings plus their sum — matching the wall-time `--json` path's one-line-per-rep shape exactly.

The timer call sites live in the example's `dijkstra` implementation (not the heap library) — Phase 2 instrumentation in the library stays untouched. Cross-language consistency is enforced by code review, not by a tool.

### Output schema

Per `(language, graph, arity)` cell, N=10 lines at `benchmarks/results/<lang>/<graph>_d<arity>.profile.jsonl` (one JSON object per line, matching the wall-time `.jsonl` shape):

```json
{
  "schema_version": 1,
  "language":       "Rust",
  "graph":          "huge_dense",
  "arity":          8,
  "rep":            6,
  "section_us": {
    "setup":    1234.5,
    "pop":      5678.9,
    "relax":   12345.6,
    "inc_pri":  3456.7,
    "total":   22715.7
  }
}
```

`total` is the sum of the four sections for that repetition — **not** the same number as the Phase 3 wall-time median. The section sum still includes timer perturbation the plain wall-time measurement doesn't pay for, so absolute values are not comparable across the two tables even though both are now medians of N=10. `benchmarks/scripts/aggregate/` reduces the 10 lines per cell to one median `SectionUs` per language (each of the five fields' median computed independently, same method as the wall-time median) before building `results/SECTIONS.md`'s tables. Cross-language *ratios* are the meaningful signal, not absolute parity with Phase 3's `wall_time_us`.

### Scope and reporting

Same 24-cell matrix as Phase 3 wall-time (8 graphs × 3 arities × 5 languages), one `.profile.jsonl` per cell (10 lines each) — 120 files, 1200 records. Headline tables in `benchmarks/results/SECTIONS.md` focus on `huge_dense × d=8`:

**Percentage of total time per section:**

| Section | Rust % | Go % | TS % | Zig % | C++ % |
|---|---:|---:|---:|---:|---:|
| setup | … | … | … | … | … |
| pop | … | … | … | … | … |
| relax | … | … | … | … | … |
| inc_pri | … | … | … | … | … |

**Absolute time per section (µs)** at `huge_dense × d=8`:

| Section | Rust | Go | TS | Zig | C++ |
|---|---:|---:|---:|---:|---:|
| relax | … | … | … | … | … |
| pop | … | … | … | … | … |
| inc_pri | … | … | … | … | … |
| setup | … | … | … | … | … |

The `relax` row is where the hashmap-attribution story should be most visible. If Rust's `relax` is 3× Go's while `pop` and `inc_pri` are within 1.2× of Go's, the SipHash + `String`-clone hypothesis is confirmed at the wall-time level. **On the run captured after adopting K=2/N=10 repetition, it was not** — see `results/SECTIONS.md`'s Finding section for the actual numbers and the IQR evidence that this isn't a repeat of the original single-sample noise problem.

### Caveats

- **Timer overhead at sub-µs scale.** Four timer pairs add ~50–200 ns per main-loop iteration. On `huge_dense` (~63 710 pop iterations) that's milliseconds of overhead — non-negligible but symmetric across languages. We report it; we don't try to subtract it. Repeating the call K+N times (per the revised Mechanism above) reduces *cross-rep* noise; it does not reduce this per-language timer-call overhead itself — see **Timer-call overhead measurement** below, which quantifies rather than merely flags this variable.
- **What this can't measure**: allocator-internal time (Go's GC, Rust's malloc), cache-miss attribution, instruction-cache vs data-cache effects. CPU-counter-grade profiling (`perf` / VTune / ETW) is the right tool for those questions; Phase 3.5 stays in user-mode wall time.
- **Multiplicative aliasing.** `pop` and `inc_pri` each contain a small `compareTo` callback inside the heap. Timing them at the call site folds the comparator cost into the heap section — intentional, since the comparator is part of the heap path.
- **Go's Windows timer-resolution floor** (documented for Phase 3 wall-time above) applies to individual section-timing reps too: a single rep's `inc_pri` (or, less often, another section) can read `0` when the real cost is simply below what `time.Since` resolves on this OS. The median across N=10 reps reduces this but does not eliminate it if more than half the reps land on the same quantized value.

### Timer-call overhead measurement

Motivation: after the K=2/N=10 revision above, the hashmap-cost hypothesis (SipHash + `String`-clone in `relax` explaining Rust's wall-time gap vs. Go) was still not confirmed — `relax`'s ratio wasn't clearly larger than `pop`'s. One remaining candidate explanation is instrumentation itself: if one language's native timer read costs meaningfully more than another's, that asymmetry could bleed into the section numbers independent of any real algorithmic difference. This measures that floor directly instead of leaving it as an unquantified caveat.

A `--timer-overhead` flag in each example binary:
- Graph/arity-independent — no `--graph`/`--arity` required.
- 10 independent trials, each 1,000,000 back-to-back calls to the same native monotonic-timer primitive used for section timing above (`Instant::now` / `time.Now` / `performance.now` / `std.Io.Clock.Timestamp.now` / `chrono::high_resolution_clock::now`).
- Per-call cost = a trial's total elapsed time ÷ 1,000,000 iterations. Per-bracket cost = per-call × 2, since each Phase 3.5 section timer pays two clock reads per section per main-loop iteration (one at bracket-start, one folded into the elapsed/duration computation at bracket-end).

Output schema: one JSON line per trial at `benchmarks/results/<lang>/timer_overhead.jsonl` (10 lines; no `graph`/`arity` fields, since the measurement is independent of both):

```json
{
  "schema_version": 1,
  "language":       "Rust",
  "trial":          6,
  "iterations":     1000000,
  "per_call_us":    0.0253,
  "per_bracket_us": 0.0506
}
```

`benchmarks/scripts/aggregate/` reduces the 10 trials to one median `per_call_us` (and IQR) per language via the same `stats::compute` method used for wall-time and section medians, and renders `results/SECTIONS.md`'s "Timer call overhead (instrument floor)" table — sorted cheapest-first, with a Ratio column normalized so the fastest language reads 1.00×.

**Finding**: Go's timer read is cheapest; TypeScript's is priciest, at roughly an order of magnitude more per call (see `results/SECTIONS.md` for the current run's exact medians and ratios). At nanoseconds per call against the Finding's microsecond-to-millisecond section totals, this confound is one to two orders of magnitude too small to be the primary explanation for the Rust-vs-Go gap on its own — real and non-zero, but not the root cause.

### Out of scope for Phase 3.5

- CPU performance counters (cache misses, branch mispredicts). Use `perf` directly when needed.
- Per-language flamegraphs as the *primary* output. Run them ad-hoc; the cross-language story lives in the section table.
- Cross-machine reproducibility. Section *ratios* should be roughly stable; absolute section times will vary with CPU/cache.

## Status checklist

Phase 3 implementation tracks against this list. Each item is a concrete deliverable. This checklist went un-updated through the v2.6.0 Phase 3 ship (the work was done, per `CHANGELOG.md`, but the boxes here were never checked) — corrected as part of the v2.7.0 Phase 3.5 pass, which also closed the two items Phase 3 shipped without (`aggregate.rs`, real README/SUMMARY.md content).

### Per-language harness (5×)
- [x] `examples/dijkstra/Cpp/` — add `--repetitions`, `--warmup`, `--json`
- [x] `examples/dijkstra/Go/` — add `--repetitions`, `--warmup`, `--json`
- [x] `examples/dijkstra/Rust/` — add `--repetitions`, `--warmup`, `--json`
- [x] `examples/dijkstra/TypeScript/` — add `--repetitions`, `--warmup`, `--json`
- [x] `examples/dijkstra/Zig/` — add `--repetitions`, `--warmup`, `--json`

### Runner scripts
- [x] `benchmarks/scripts/<lang>/run.ps1` (Windows) per language — **`run.sh` (Unix) was never built**; this project's benchmark tooling is Windows-only in practice (every `env-*.json` records a Windows host). Left honestly unchecked rather than claimed.
- [ ] `benchmarks/scripts/run-all.{ps1,sh}` — never built; each language's `run.ps1` is invoked independently instead. Not a blocker (5 explicit invocations vs. one orchestrator is a minor convenience gap), but genuinely not done.
- [x] `benchmarks/scripts/env.ps1` — captures `env-<Language>.json` once per machine (no `.sh` counterpart, same Windows-only caveat as above).

### Aggregation
- [x] `benchmarks/scripts/aggregate/` — Rust binary crate (methodology.md's `.rs` option). JSONL/stats/RSS/profile → tables + invariant check.
- [x] `benchmarks/results/<lang>/*.jsonl` populated for all 120 cells
- [x] `benchmarks/results/<lang>/*.stats.json` populated for all 120 cells
- [x] `benchmarks/results/<lang>/huge_dense_d{2,4,8}.rss.json` populated (15 records)

### Documentation
- [x] `benchmarks/README.md` — "Indicative timings" replaced with Phase 3 tables; "Phase 3 — Benchmark infrastructure (planned)" → "completed."
- [x] `benchmarks/results/SUMMARY.md` — full 120-cell matrix
- [x] `CHANGELOG.md` — Phase 3 entry under Unreleased (backfilled in v2.7.0; Phase 3's own v2.6.0 CHANGELOG entry only covered Phases 1–2's instrumentation, not the wall-time/stats/RSS results Phase 3 itself produced)
- [x] `ROADMAP.md` — check Phase 3 boxes (already checked in ROADMAP.md's v2.6.0 section — this methodology.md checklist was the only place still stale)

### Phase 3.5 (v2.7.0)
- [x] `--profile-sections` flag in all 5 Dijkstra examples
- [x] Per-section timers (`setup`/`pop`/`relax`/`inc_pri`) in each example's `dijkstra` implementation
- [x] `benchmarks/scripts/<Lang>/run.ps1` Pass 4 — `.profile.jsonl` for all 120 cells (10 lines each, K=2/N=10 revision)
- [x] `benchmarks/results/SECTIONS.md` — cross-language attribution tables at `huge_dense × d=8`
- [x] `CHANGELOG.md` entry + the answer to "where does the extra time go?" — revised from a single-call design (result looked like noise) to K=2/N=10 repetition (median + IQR); the answer held under the more rigorous measurement: not cleanly `relax` — see `SECTIONS.md`'s Finding/Caveats
- [x] `--timer-overhead` flag in all 5 Dijkstra examples + `benchmarks/results/<lang>/timer_overhead.jsonl` (10 trials each) — quantifies the instrument-floor confound named in Phase 3.5's Caveats; see `SECTIONS.md`'s "Timer call overhead" table
