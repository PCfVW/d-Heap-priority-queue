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
- Per-section timing *inside* `dijkstra` (hash-map vs heap-op vs comparator). Phase 2 cost-per-comparison strongly suggests Rust's `String` clones + SipHash and Zig's `std.StringHashMap` dominate, but attribution requires profiler runs we are not signing up for here.
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

## Status checklist

Phase 3 implementation tracks against this list. Each item is a concrete deliverable.

### Per-language harness (5×)
- [ ] `examples/dijkstra/Cpp/` — add `--repetitions`, `--warmup`, `--json`
- [ ] `examples/dijkstra/Go/` — add `--repetitions`, `--warmup`, `--json`
- [ ] `examples/dijkstra/Rust/` — add `--repetitions`, `--warmup`, `--json`
- [ ] `examples/dijkstra/TypeScript/` — add `--repetitions`, `--warmup`, `--json`
- [ ] `examples/dijkstra/Zig/` — add `--repetitions`, `--warmup`, `--json`

### Runner scripts
- [ ] `benchmarks/scripts/<lang>/run.ps1` (Windows) and `run.sh` (Unix) per language
- [ ] `benchmarks/scripts/run-all.{ps1,sh}` — orchestrates wall-time + stats + RSS passes
- [ ] `benchmarks/scripts/env.{ps1,sh}` — captures `env.json` once per machine

### Aggregation
- [ ] `benchmarks/scripts/aggregate.py` (or `.rs`) — JSONL → tables + invariant check
- [ ] `benchmarks/results/<lang>/*.jsonl` populated for all 120 cells
- [ ] `benchmarks/results/<lang>/*.stats.json` populated for all 120 cells
- [ ] `benchmarks/results/<lang>/huge_dense_d{2,4,8}.rss.json` populated (15 records)

### Documentation
- [ ] `benchmarks/README.md` — "Indicative timings" replaced with Phase 3 tables; "Phase 3 — Benchmark infrastructure (planned)" → "completed."
- [ ] `benchmarks/results/SUMMARY.md` — full 120-cell matrix
- [ ] `CHANGELOG.md` — Phase 3 entry under Unreleased
- [ ] `ROADMAP.md` — check Phase 3 boxes
