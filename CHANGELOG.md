# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Convention** (from v2.7.0, with the ~4–6 week minor-version cadence): every user-visible change adds a line to `[Unreleased]` *at the time the change lands*, not retroactively at release. Internal refactors that don't affect users may be omitted.

## [Unreleased]

### Added

- **`.github/workflows/ci.yml`**: preflight build + lint + test for all 5 languages (Rust: `cargo fmt --check` + `clippy::pedantic` + `cargo test`; Go: `gofmt` + `go vet` + `go test`; TypeScript: `eslint` + `tsc --noEmit` + `vitest`; Zig: `zig fmt --check` + `zig build test`; C++: GCC 13 + CTest), on every push/PR to `master` plus a weekly cron canary for toolchain drift. Added post-v2.7.0 — the release automation shipped in that same cycle with no preflight gate, and while it wouldn't have caught either bug below directly (both are release-time-only failure modes), a broken commit sitting undetected until the next tag is exactly the gap this closes.

### Changed

- **`publish-npm.yml`**: switched from a static `NPM_TOKEN` secret to npm's OIDC **trusted publishing** (`permissions: id-token: write`, an explicit `npm install -g npm@latest` since trusted publishing needs npm CLI ≥ 11.5.0 and Node 20's bundled npm predates that) — no publish secret to leak or rotate. Configured on npmjs.com's package Settings → Trusted Publisher, pointing at this repo's `publish-npm.yml`.
- **`publish-crates.yml`**: switched from the `CARGO_REGISTRY_TOKEN` secret to crates.io **trusted publishing** (`environment: crates-io`, `permissions: id-token: write`, `rust-lang/crates-io-auth-action@v1` for a short-lived token revoked when the job ends). crates.io's d-ary-heap Settings → Trusted Publishing points at this repo's `publish-crates.yml` with environment `crates-io`, and *Require trusted publishing for all new versions* is on, so API tokens are refused. The auth step runs before the "already published?" check, so a manual `workflow_dispatch` run on a published version proves the setup without publishing. Both `CARGO_REGISTRY_TOKEN` and the now-unused `NPM_TOKEN` secrets are deleted. (#2)
- **All workflows**: actions moved off their Node.js 20 majors, which GitHub Actions deprecates: `actions/checkout` v4 → v7, `actions/setup-node` v4 → v7, `actions/setup-go` v5 → v7, `actions/upload-pages-artifact` v3 → v5, `actions/configure-pages` v4 → v6, `actions/deploy-pages` v4 → v5. (#4, #5)
- **`ci.yml`, `deploy-demo.yml`**: TypeScript and the demo now build on Node 24 (was 20, end of life in April 2026), matching `publish-npm.yml`.
- **`TypeScript/src/PriorityQueue.ts`**: the 13 non-null assertions' `// INDEX:` comments are now `// eslint-disable-line @typescript-eslint/no-non-null-assertion -- INDEX: ...` directives, so `npm run lint` and CI are silent while every assertion is justified, and a new unjustified `!` stands out as the only warning. `TypeScript/CONVENTIONS.md` updated to match.

### Fixed

- **`publish-npm.yml`/`publish-crates.yml` never actually ran on release**: discovered immediately during the v2.7.0 ship — both were triggered by `release: types: [published]`, but `create-release.yml` creates that Release using the default `GITHUB_TOKEN`, and GitHub's anti-recursion rule blocks a `GITHUB_TOKEN`-authored event from triggering other workflows (silently — no failure, no red X, just nothing happens). Re-pointed both at `push: tags: v*` — the same trigger `create-release.yml` itself uses successfully, since that fires from a real, human-authored push — plus `workflow_dispatch` as a manual escape hatch. v2.7.0 itself had to be published to crates.io/npm via a manual `workflow_dispatch` run after the fix landed, since the original tag push predates it.
- **`publish-crates.yml`'s "already published?" check misread crates.io**: it called the crates.io API without a User-Agent, which crates.io answers with 403; the check took that for "not published" and ran `cargo publish`, which crates.io then refused (`d-ary-heap@2.7.0 already exists`). The check now sends a User-Agent naming the workflow, and fails the job on any status other than 200 (skip) or 404 (publish) instead of guessing. (#3)

## [2.7.0] - 2026-08-31

**Released to [crates.io](https://crates.io/crates/d-ary-heap) (Rust), [npm](https://www.npmjs.com/package/d-ary-heap) (TypeScript), and [pkg.go.dev](https://pkg.go.dev/github.com/PCfVW/d-Heap-priority-queue/Go/v2) (Go) via the new `create-release.yml`/`publish-*.yml` automation this same release introduces; C++ and Zig distribute via the GitHub repo source and git tag.**

*Automation & Per-Section Timing.* CI/CD workflows automate the ship steps v2.6.0 still did by hand; Phase 3.5 attempts to attribute Phase 2's Rust-vs-Go wall-time gap to a specific section of `dijkstra()`, and is honest that a shared, non-quiesced sandbox couldn't settle the question.

### Added

- **CI/CD automation** (v2.7.0, lands first): `.github/workflows/create-release.yml` (on `v*` tag push, extracts the matching `CHANGELOG.md` section via a literal-match awk script and publishes a GitHub Release immediately, then creates/pushes the matching `Go/vX.Y.Z` tag and nudges the Go module proxy), `.github/workflows/publish-npm.yml` and `.github/workflows/publish-crates.yml` (on Release published: test, then publish, each skipping gracefully — no red X — if the local version is already live on its registry, so a future release that doesn't bump every language's version stays safe).
- **`Go/CONVENTIONS.md`, `Cpp/CONVENTIONS.md`, `zig/CONVENTIONS.md`, `TypeScript/CONVENTIONS.md`**: adapted from Amphigraphic-Strict's Cog/Stoic/Gizmo/Terse respectively, matching `Rust/CONVENTIONS.md`'s structure (Trigger Checklist → "When Writing X" rule sections → testing conventions → honest Lint Floor). Each is grounded in the language's actual source rather than a generic copy of the upstream rules.
- **Phase 3.5 — per-section timing**: a `--profile-sections` flag in all 5 Dijkstra examples (`examples/dijkstra/{Cpp,Go,Rust,TypeScript,Zig}/`), each gaining a `SectionTimings` type and a dedicated `dijkstraProfiled`/`DijkstraProfiled`/`dijkstra_profiled` function (kept separate from the Phase 3 wall-time/`--stats` hot paths, so those are unaffected). Every language wraps the identical four sections (`setup`/`pop`/`relax`/`inc_pri`) around the identical lines of the algorithm, per `benchmarks/methodology.md`'s cross-language contract — the `relax`/`inc_pri` split required hoisting an explicit `improved` boolean in every language so the timer boundary doesn't re-evaluate a comparison against data it just mutated. Requires `--arity=<d>` (mirrors `--report-rss`); reuses the same `--warmup=K`/`--repetitions=N` flags the wall-time `--json` path already has, emitting one JSON line per timed rep (methodology.md's `section_us` schema plus a `rep` field, matching the wall-time `.jsonl` shape). Native per-language timer: `Instant::now` (Rust), `time.Now`/`time.Since` (Go), `performance.now()` (TypeScript), `std.Io.Clock.Timestamp` (Zig — `std.time.nanoTimestamp` no longer exists in 0.16), `std::chrono::high_resolution_clock` (C++).
- **`benchmarks/scripts/<Lang>/run.ps1`**: Pass 4 (per-section timing), sweeping the full 24-cell matrix per language with `--warmup=2 --repetitions=10` (120 `.profile.jsonl` files total, 10 lines each) via `--profile-sections`.
- **`benchmarks/scripts/aggregate/`**: new standalone Rust binary crate (mirrors `graphgen`'s pattern) that reads every `benchmarks/results/**/*.{jsonl,stats.json,rss.json,profile.jsonl}`, verifies the Phase 2 comparison-count invariant (non-zero exit on disagreement), and emits `benchmarks/README.md`'s condensed wall-time tables, `benchmarks/results/SUMMARY.md` (full 120-cell matrix with median/IQR/min/max), and `benchmarks/results/SECTIONS.md` (Phase 3.5 headline tables at `huge_dense × d=8`, each section's value the median of its 10 sampled reps). This is the `aggregate.py`/`.rs` tool `methodology.md` called for during Phase 3 but that never got built — closed now as part of backfilling Phase 3's own documentation gap (see Changed, below).
- **`--timer-overhead` flag** in all 5 Dijkstra examples: quantifies the per-language native-timer-call cost itself, to check whether it could be confounding the Phase 3.5 section-timing story. Graph/arity-independent — 10 trials, each 1,000,000 back-to-back calls to the same native monotonic timer used for section timing (`Instant::now` / `time.Now` / `performance.now` / `std.Io.Clock.Timestamp.now` / `chrono::high_resolution_clock::now`), emitting one JSON line per trial (`benchmarks/results/<lang>/timer_overhead.jsonl`, `per_call_us`/`per_bracket_us`). `aggregate` reduces this to a median-per-language table in `SECTIONS.md` ("Timer call overhead (instrument floor)"), sorted cheapest-first with a Ratio column (1.00× = fastest). **Finding**: Go is cheapest (~4.6 ns/call), TypeScript costliest (~43 ns/call, 9.4× Go) — real and systematic, but one to two orders of magnitude too small against the Finding's microsecond-to-millisecond section totals to be the primary driver of the Rust-vs-Go gap on its own. Documented in `benchmarks/methodology.md`'s new "Timer-call overhead measurement" subsection under Phase 3.5.
- **`pop_count`/`relax_count` plain counters** (no timer) added to `--profile-sections` in all 5 languages, alongside the existing timing brackets, to directly verify the `~|V|`/`~|E|` bracket-count assumption used when reasoning about the timer-overhead confound above, instead of assuming it. `aggregate` gains `check_call_count_invariant` — mirroring the Phase 2 comparison-count invariant — confirming the counts are identical across all 10 reps *and* all 5 languages (a divergence would be a bug, not variance), and renders a new "Call-count-based overhead attribution" table in `SECTIONS.md`. **Confirmed**: `pop_count`=2,500, `relax_count`=125,000 on `huge_dense × d=8`, exactly matching the assumption. The attribution itself is inconclusive rather than a clean answer: `pop`'s instrumentation-asymmetry prediction (~104 µs) is too small to interpret meaningfully against a measured gap whose *sign* varies run to run at that magnitude; for `relax`, the same per-bracket difference extrapolated linearly over its ~125,000 calls *overshoots* the actual measured gap — evidence that `--timer-overhead`'s isolated, back-to-back measurement does not transfer linearly to a timer call interleaved with real hashmap/heap work. Documented in `benchmarks/methodology.md`'s new "Call-count verification" subsection under Phase 3.5.

### Changed

- **`Rust/CONVENTIONS.md`**: audited against Grit; adds a `# Panics` doc-section rule (mirroring the existing `# Errors` one), documents the crate's deliberate lifetime-elision deviation from Grit Rule 1 (fighting `clippy::needless_lifetimes` otherwise), and records the intentional `thiserror`-free `Error` enum as a documented deviation from Grit Rule 8.
- **`TypeScript/eslint.config.js`**: fixes a dead rule key (`explicit-function-return-types`, plural — not a real rule, silently ignored), promotes `no-explicit-any` to `'error'`.
- **`TypeScript/src/PriorityQueue.ts`, `TypeScript/src/instrumentation.ts`**: the 13 existing non-null assertions and one type assertion now carry `// INDEX:`/`// ASSERT:` justification comments, per the new `TypeScript/CONVENTIONS.md` convention (mirrors Rust's `INDEX:`/`CAST:` annotations).
- **`Cpp/PriorityQueue.h`**: `front()`'s existing UB-on-empty comment reworded to carry the formal `UB:` marker, per the new `Cpp/CONVENTIONS.md` convention.
- **`benchmarks/README.md`**: the v2.6.0 "Indicative timings" ballpark placeholder — never replaced when Phase 3 actually shipped — is now real Phase 3 data (four condensed tables: sparse / dense / grid / `huge_dense`), generated by `aggregate`. Status table's Phase 3 row flipped to Done, Phase 3.5 row added, "Phase 3 — Benchmark infrastructure (planned)" section rewritten to reflect what shipped, directory structure and Contributing blurb refreshed. The "Cost per heap comparison" ns/cmp table's closing caveat now points at `results/SECTIONS.md`'s real per-section attribution instead of a forward reference to "Phase 3 per-section timing" (that work is Phase 3.5, and its answer is now in).
- **`benchmarks/methodology.md`**: Status checklist backfilled — most Phase 3 boxes were left unchecked despite v2.6.0 shipping the work; honestly still-unchecked where genuinely not done (`run.sh`/`run-all.{ps1,sh}` Unix/orchestrator variants were never built — this project's benchmark tooling is Windows-only in practice). New Phase 3.5 checklist section, fully checked.
- **Phase 3.5 mechanism revised mid-cycle**: the original design (single un-iterated `--profile-sections` call, no warmup, no repetitions) produced a result that looked like measurement noise rather than signal — Rust's heap-only `pop` section showed a *larger* relative gap vs. Go (3.01×) than the hashmap-heavy `relax` section (1.13×), the opposite of what the SipHash/`String`-clone hypothesis predicts. Rather than defend the single-call design, `--profile-sections` now reuses Phase 3's own K=2 warmup / N=10 repetitions protocol (see Added, above), and `benchmarks/methodology.md`'s "Phase 3.5" section, `results/SECTIONS.md`, and `ROADMAP.md`'s Phase 3.5 line are all updated to match. **The re-measurement changed the confidence, not the conclusion**: total-time IQR across the 10 reps came in comfortably under 15% of the median for both Rust and Go, and `relax` (1.31× Go's) is still not clearly larger than `pop` (1.52× Go's) — the hashmap-cost hypothesis is not confirmed even under a much more stable measurement. Two caveats the repetition doesn't resolve: Go's Windows `time.Since` timer-resolution floor still under-reports fast sections on individual reps, and per-language timer-call overhead itself remains unmeasured (repetition reduces cross-rep noise, not that overhead).
- **The Phase 3.5 Finding is measurement-condition-sensitive, not settled**: regenerating all 120 `.profile.jsonl` files to add the `pop_count`/`relax_count` counters (see Added, above) required re-running the full K=2/N=10 capture, and that re-run's numbers flipped the qualitative answer versus the capture described in the bullet directly above — this run's `relax` (1.59× Go's) *is* clearly larger than `pop` (0.91×), the opposite of "not confirmed." Both captures used the identical protocol on the identical (shared, non-quiesced, "Balanced"-power-plan) sandbox; the flip itself is the finding — a single capture's qualitative Phase 3.5 conclusion should be read as directionally suggestive for *this measurement environment*, not as a settled answer, until re-run on the dedicated quiesced rig methodology.md recommends. `results/SECTIONS.md` always reflects whichever capture produced the files currently on disk; the historical bullets in this CHANGELOG are frozen snapshots of what a specific run showed and are intentionally not rewritten when a later run changes the numbers.

### Fixed

- **Zig 0.16 toolchain compatibility** (`zig/`, `examples/dijkstra/Zig/`) — unrelated to Phase 3.5, but blocking: this sandbox's installed Zig (0.16.0) is ahead of the project's declared `minimum_zig_version` (was `0.15.2`, matching the v2.6.0 CHANGELOG's "Zig 0.15.2"), and the existing library/example no longer compiled. Fixed and bumped to `0.16.0`: `std.heap.GeneralPurposeAllocator` → `DebugAllocator` (~60 call sites); `ArrayList(u8).writer()` removed, `toString()` now appends directly; `main()` now takes `std.process.Init` for `.gpa`/`.io` instead of a manually-constructed allocator plus the removed `std.process.argsWithAllocator`; every filesystem call (`std.fs.cwd`, `File.stdout`, file read/write) moved from `std.fs`/`std.fs.File` to `std.Io.Dir`/`std.Io.File`, threading an explicit `io: std.Io` parameter; `std.time.nanoTimestamp` removed, replaced by `std.Io.Clock.Timestamp`; `std.mem.trimRight` renamed to `trimEnd`; Win32 `BOOL` is now the distinct `Bool(c_int)` type, not directly comparable to `0`. All 64 library tests and every existing example CLI mode re-verified after the fix.

## [2.6.0] - 2026-05-11

**Released to [crates.io](https://crates.io/crates/d-ary-heap) (Rust) and [npm](https://www.npmjs.com/package/d-ary-heap) (TypeScript); Go module surfaced via `go/v2.6.0` tag on [pkg.go.dev](https://pkg.go.dev/github.com/PCfVW/d-Heap-priority-queue/Go/v2); C++ and Zig distribute via the GitHub repo source.**

*Instrumentation & Benchmarks across all five languages.* Phase 1 (test graph corpus + tooling + GRAMMAR spec), Phase 2 (cross-language comparison-count instrumentation in C++, Go, Rust, TypeScript, Zig), and Phase 3 (benchmark runners + results). The 5-way Phase 2 invariant is verified byte-for-byte across all 24 (graph, arity) cells: every language produces identical comparison counts, isolating cross-language wall-time differences to per-operation cost.

### Added

#### Phase 3 harnesses (all five languages) + methodology spec
- **`benchmarks/methodology.md`**: reproducibility contract for Phase 3. Pins down the 3-pass protocol (wall-time / comparison-counts / RSS), N=10 reps with K=2 warmup, median + IQR + min + max statistics, build-flag matrix, environment-block schema, and per-cell JSON output formats. Doubles as the Phase 3 implementation framing doc — the bottom-of-doc Status checklist is the per-language todo list.
- **`examples/dijkstra/Rust/`**: gains `--warmup`, `--repetitions`, `--arity`, `--json`, `--env-file`, `--report-rss` flags. `--json` switches the binary from human-readable output to JSONL records on stdout (one record per timed `dijkstra` call). `--report-rss` emits a single peak-RSS JSON record via `GetProcessMemoryInfo` (Windows-only stub on other platforms; cfg-gated `windows-sys` dep). Three `dijkstra` calls in the benchmark paths wrapped in `std::hint::black_box` to defeat any future LLVM elision. Source-compatible with all existing `cargo run --release -- --graph=…` invocations.
- **`examples/dijkstra/Go/`**: matching six flags via stdlib `flag` package. `--json` emits struct-ordered records (logical field order, unlike Rust's alphabetical `serde_json` default). RSS read via direct `syscall.NewLazyDLL("psapi.dll")` → `GetProcessMemoryInfo` (no `golang.org/x/sys` dep needed; the package does not expose this Win32 API). Split across `rss_windows.go` / `rss_other.go` build-tagged files. The Go binary preserves the existing single-shot human-mode output for `--graph=… --quiet` users.
- **`benchmarks/scripts/env.ps1`**: PowerShell script captures the machine `env` block (OS, CPU model + base GHz, logical cores, RAM, power plan, toolchain, build-flags, commit, date). Takes a `-Language` parameter (Rust | Go | TypeScript | Zig | Cpp) and writes to `benchmarks/scripts/env-<Language>.json`; per-language file keeps toolchain/build_flags clean across runs. TypeScript / Zig / C++ branches are stubbed pending those ports.
- **`benchmarks/scripts/Rust/run.ps1`** and **`benchmarks/scripts/Go/run.ps1`**: orchestrate the 3-pass capture per language. Pass 1 (wall-time, `--warmup=2 --repetitions=10`), Pass 2 (`--stats`), Pass 3 (`--report-rss` on `huge_dense` only). Idempotent. `-SmokeTest` runs one cell (`huge_dense × d=8`); `-SkipBuild` reuses an existing built binary.
- **`examples/dijkstra/TypeScript/`**: same six flags via the existing hand-rolled `parseArgs()` in `src/index.ts`. JSON output via `JSON.stringify` of plain object literals — JS preserves insertion order for string keys (ES2015+), giving logical field order natively. Peak RSS via Node's built-in `process.resourceUsage().maxRSS`, which on Windows is implemented inside Node via the same `GetProcessMemoryInfo` / `PeakWorkingSetSize` Win32 call we routed through `windows-sys` (Rust) and `syscall.NewLazyDLL("psapi.dll")` (Go) — **zero native binding, zero syscall plumbing** in the TS harness. Library is unchanged; the v2.4.0 `instrumentComparator` + `onBeforeOperation`/`onAfterOperation` hooks plus the v2.6.0 commit `86c7fe7` `--stats` wiring already gave Phase 3 everything it needs.
- **`examples/dijkstra/Zig/`**: same six flags via the existing hand-rolled `parseArgs` in `src/main.zig`. JSON output via `std.fmt.bufPrint` + `std.fs.File.stdout().writeAll` (Zig 0.15 reworked `std.io`; `getStdOut()` no longer exists — `std.fs.File.stdout()` is the new entry point). The env block is **spliced verbatim** into each wall-time record rather than parsed and re-emitted, which exposed a latent issue: PowerShell's default `ConvertTo-Json` is pretty-printed (multi-line), so `env.ps1` now uses `-Compress` to keep `env-<Language>.json` single-line and preserve the "one record per line" JSONL invariant. Rust / Go / TypeScript don't care (they re-encode env via their JSON libraries) but Zig's byte-level splice did. Peak RSS via manual `extern "psapi"` / `extern "kernel32"` declarations (Zig 0.15.2's `std.os.windows` doesn't expose `GetProcessMemoryInfo`), built-tagged via conditional `@import("rss_windows.zig")` / `@import("rss_other.zig")`. `extern "psapi"` auto-linked at build time — no `exe.linkSystemLibrary("psapi")` needed in `build.zig`. Library is unchanged; the v2.6.0 commit `fa2d3fe` Phase 2 instrumentation already shipped the comptime-bool `collect_stats` parameter.
- **`examples/dijkstra/Cpp/`**: same six flags via the existing hand-rolled `parse_args` (uses C++23 `std::string_view::starts_with` + `std::optional`). JSON output via `std.format` (already in use for the human `--stats` path); each record built as a single `std::format(...)` call with embedded `{{ }}` brace escapes. The env block is spliced verbatim (matches Zig's pattern). Peak RSS via **native** `<windows.h>` + `<psapi.h>` + `GetProcessMemoryInfo`; the cleanest of the five — no syscall wrapping, no extern declarations, no LazyDLL, no native-binding shim. `CMakeLists.txt` gains a Windows-only `target_link_libraries(dijkstra PRIVATE psapi)` block. The existing two-path stats wiring (`if (args.stats) InstrumentedPriorityQueue ... else PriorityQueue ...`) is reused unchanged. Library is unchanged; the v2.6.0 commit `983df98` Phase 2 instrumentation already shipped the template policy class with `[[no_unique_address]]` zero-bytes elision.
- **`benchmarks/results/{Rust,Go,TypeScript,Zig,Cpp}/`**: 51 files each (24 wall-time `.jsonl` + 24 `.stats.json` + 3 `huge_dense_d{2,4,8}.rss.json`). **5-way cross-language Phase 2 invariant verified on every cell** — Rust, Go, TypeScript, Zig, and C++ produce byte-for-byte identical `total` comparison counts across all 24 (graph, arity) cells. This is the strongest cross-language consistency signal in the project: five very different languages with five very different hash-map implementations (Rust's SipHash + `String`-clone, Go's flat-bucket AES-NI, V8's `Map<string,…>`, Zig's Wyhash `StringHashMap`, MSVC's chained-bucket `std::unordered_map`) all agreeing exactly.
- **Cross-language wall-time picture at d=8** (single Windows 11 machine, AMD Ryzen 9 5950X, balanced power plan, MSVC 19.44.35225, Node v24.11.1, Go 1.25.5, rustc 1.95.0, Zig 0.15.2). On `huge_dense`: **Go 11 016 µs, Zig 12 534 µs, TypeScript 13 444 µs, C++ 15 142 µs, Rust 22 455 µs** — Go is fastest, Rust 2× slower than Go. On `large_*` cells at d=8: **Zig is *fastest*** (846 µs sparse / 899 µs grid / 2854 µs dense), beating Go by 5–20%; C++ comes in 4th throughout, ~1.5–2× faster than Rust but ~1.4× slower than Zig. The Phase 1 indicative `huge_dense × d=8` Zig median of 98 443 µs was simply wrong — Phase 3's 12 534 µs (an 8× drop) is consistent with Phase 2's cost-per-comparison value of 157 ns/cmp × 75 936 cmps ≈ 11.9 ms. Phase 1 indicative for C++ (14 868 µs) was within 1.8% of Phase 3 (15 142 µs); Rust within 3.4%; Go within 10%; TypeScript within 18%; Zig was the only language where Phase 1 was qualitatively wrong. **The Phase 2 puzzle (Go fastest, TypeScript surprisingly close to C++/Rust, Rust slowest) holds and refines**: TypeScript is ~1.7–2× faster than Rust on every `large_*` / `huge_dense` cell at d=8 (V8's `Map<string,…>` outperforms Rust's SipHash+`String`-clone `HashMap<String,…>`); Zig joins Go in the fast tier rather than the slow one as Phase 1 had suggested. The `medium_*` and `small_*` Go cells hit the Windows `time.Since` resolution floor and read 0 µs, per the timer-resolution caveat in `methodology.md`. TypeScript IQR on `medium_*` cells reaches 70–84% (V8 JIT tier-up); K=5 warmup did not measurably tighten variance vs K=2.

#### Test graph corpus + reproducible generator (Phase 1)
- **`benchmarks/scripts/graphgen/`**: standalone Rust crate (`petgraph` + `rand_chacha::ChaCha8Rng`) that generates the canonical graph corpus from `benchmarks/graphs.toml`. Provides `generate` and `verify` subcommands; `verify` re-emits each graph and `diff`s byte-for-byte against the committed JSON.
- **`benchmarks/graphs.toml`**: declarative spec of all corpus graphs with seeds + weight ranges.
- **`examples/dijkstra/graphs/GRAMMAR.md`**: ISO/IEC 14977 EBNF for the constrained, deterministic JSON subset the corpus uses, plus byte-level canonical-output spec.
- **Seven corpus graphs** (committed to git): `medium_sparse` (n=100, |E|=200), `medium_dense` (n=100, |E|=1000), `medium_grid` (10×10 lattice, |E|=360), `large_sparse` (n=1000, |E|=2000), `large_dense` (n=1000, |E|=31623), `large_grid` (32×32 lattice, |E|=3968), `huge_dense` (n=2500, |E|=125000, ~6 MB JSON). Erdős–Rényi graphs are connectivity-enforced (every vertex reachable from `v0`).
- **`--graph=NAME`, `--source`, `--target`, `--quiet`** CLI flags in all five Dijkstra examples (TypeScript, Go, Rust, Zig, C++). Default endpoints: `A→F` for `small`, `v0 → v(N-1)` otherwise.
- **C++ `graph_parser.h`** (~150 LOC, zero dependencies): strict parser conforming to GRAMMAR.md; rejects out-of-order keys, non-ASCII vertex IDs, float weights, missing fields, trailing content. Companion `Cpp/test_graph_parser.cpp`: 18 fixture cases (5 valid + 5 rejection + 7 corpus graphs round-trip).

#### Cross-language comparison-count instrumentation (Phase 2)
- **C++**: 5th defaulted template parameter `TStats = NoOpStats` on `PriorityQueue<T, …>`; new `InstrumentedPriorityQueue<T, …>` alias for the `ComparisonStats` variant. RAII `OperationGuard` brackets each public mutator; private `compare_` helper at three comparison sites (`bestChildPosition`, `moveUp`, `moveDown`).
- **C++ tests** (`Cpp/test_instrumentation.cpp`): 12 cases including 3 compile-time `static_assert`s that prove zero-bytes elision via `[[no_unique_address]]` (or `[[msvc::no_unique_address]]` on MSVC) — `sizeof(InstrumentedPriorityQueue<int>) - sizeof(PriorityQueue<int>) == sizeof(ComparisonStats)` (152 − 104 = 48 bytes on MSVC).
- **Go**: `Options.Stats *Stats` opt-in field on heap construction; nil-by-default. Conditional bracketing via `if pq.stats != nil { startOperation(OpX); defer endOperation() }` on each public mutator.
- **Go tests** (`Go/src/instrumentation_test.go`): 10 cases including a lockstep pop-order check (default vs instrumented heaps must produce identical pop sequences) and a nil-safe-`Total()`/`Reset()` regression test.
- **Rust**: 5th defaulted template parameter `S = NoOpStats` on `PriorityQueue<T, C, S>`; new `InstrumentedPriorityQueue<T, C>` alias for the `ComparisonStats` variant. Constructor split: `PriorityQueue::new(d, c)` and `with_first(d, c, t)` on the NoOpStats impl; `with_stats(d, c)` on the ComparisonStats impl. Required because Rust's defaulted type parameters do not feed into method resolution — a single generic-over-S `new` would force every existing doctest to add a type annotation. Closure-based `bracket(op, |s| ...)` instead of an RAII guard (the borrow checker rejects guards that hold `&self.stats` across the body's `&mut self` heap-method calls). With `S = NoOpStats`, monomorphization elides every trait call; the `stats` field collapses to zero bytes via Rust's standard ZST layout — no attribute needed (unlike C++'s `[[no_unique_address]]`).
- **Rust tests** (`Rust/tests/instrumentation.rs`): 11 cases including a lockstep pop-order check, a runtime size-equality assertion as Rust's analog to C++'s `static_assert` (`size_of::<NoOpStats>() == 0`, `size_of::<PriorityQueue<…>>() == size_of::<PriorityQueue<…, NoOpStats>>()`, and the size delta to the `ComparisonStats` variant equals exactly `size_of::<ComparisonStats>()`).
- **Zig**: new module `zig/src/instrumentation.zig` (`OperationType` enum + `ComparisonStats` struct with `startOperation` / `endOperation` / `countComparison` / `total` / `reset`). `DHeap(T, Context, Comparator)` now delegates to a 4-arg `DHeapWithStats(T, Context, Comparator, comptime collect_stats: bool)` whose `stats` field is typed `if (collect_stats) ComparisonStats else void` — the un-instrumented heap's footprint is unchanged because Zig collapses `void` fields to zero bytes (the language-level analog of Rust's ZST and C++'s `[[no_unique_address]]`). Convenience alias `InstrumentedDHeap` mirrors Rust's `InstrumentedPriorityQueue`. Private `compare()` helper at three comparison sites (`bestChildPosition`, `moveUp`, `moveDown`); `if (collect_stats) self.stats.startOperation(.X); defer if (collect_stats) self.stats.endOperation();` brackets each public mutator. With `collect_stats=false` every guard is a comptime constant — branches eliminated entirely, generated code identical to the un-instrumented library.
- **Zig tests** (`zig/src/tests/instrumentation_tests.zig`, wired into `zig build test` as a second compile unit): 9 cases including a lockstep pop-order check between `DHeapItem` and `InstrumentedDHeap(Item, …)`, and a comptime-friendly size assertion `@sizeOf(DHeapItem) < @sizeOf(InstrumentedDHeap(Item, …))` that mirrors Rust's `default_stats_member_is_zero_sized`.
- **`--stats` CLI flag** in all five Dijkstra examples (TypeScript, C++, Go, Rust, Zig); per-arity print format byte-for-byte identical across the five languages. (TypeScript: the heap library shipped instrumentation in v2.4.0 but the Dijkstra example only exposed it via `--stats` later — `examples/dijkstra/TypeScript/src/dijkstra.ts` gains a `dijkstraInstrumented` companion that wraps the comparator with `instrumentComparator` and wires the `onBeforeOperation` / `onAfterOperation` hooks. Zig: `examples/dijkstra/Zig/src/dijkstra.zig` adds a `dijkstraInstrumented` companion returning `DijkstraInstrumentedResult { result, stats }`; the algorithm body is shared with `dijkstra` via a private generic `runDijkstra(comptime PQ: type, …)` helper. All five languages now follow the same "split + private generic body" pattern.)
- **Documentation of ownership-model asymmetry** in `Cpp/PriorityQueue.h`, `Go/src/instrumentation.go`, and `Rust/src/instrumentation.rs`: C++ owns the stats by value (zero-bytes via `[[no_unique_address]]`); Rust also owns by value (zero-bytes via ZST layout for `NoOpStats`); Zig also owns by value (zero-bytes via `void` field type, gated by a `comptime bool`); Go has user-owned storage with a `*Stats` pointer in the heap. The C++/Rust/Zig convergence reflects that all three have compile-time mechanisms for zero-bytes empty members (templates, monomorphisation, comptime); the Go split is forced by Go not having that mechanism.

#### Project documentation
- **`Rust/CONVENTIONS.md`**: synthesis of the Grit-base conventions adapted to a generic data-structure crate (no async / I/O / FFI / ML overlays). Lists target lint floor and surfaces drift between convention and live code.
- **Indicative cross-language timings table** in `benchmarks/README.md`: median-of-5 single-shot runs for 5 languages × 5 graphs × 3 arities (d=2/4/8). Honestly framed as ballpark only — Phase 3 will replace with proper benchmark methodology.
- **CI/CD automation section** in `ROADMAP.md`: planned npm publish + GitHub Release workflows.

### Changed

#### Library
- **C++ `PriorityQueue<T, …>`**: gains a 5th defaulted template parameter `TStats = NoOpStats` (source-compatible with all existing user code). Three comparison sites now route through `compare_(a, b)` which inlines to a bare comparator call under `NoOpStats`.
- **Go `PriorityQueue[T, K]`**: gains `Options.Stats *Stats` and `pq.Stats() *Stats` accessor. Three comparison sites now route through `pq.compareLess(a, b)` which compiles to a single nil check + comparator call.
- **Go `(*Stats).Total()` and `(*Stats).Reset()`** are nil-safe (Go's method-on-nil-receiver pattern), so `pq.Stats().Total()` works for default-heap users and matches C++'s `pq.stats().total()` semantics.
- **Rust crate**: hardening pass per `Rust/CONVENTIONS.md`. Adds `#![forbid(unsafe_code)]` and `#![deny(warnings)]` at the crate root, `#[non_exhaustive]` on `Error`, `const fn` on `d`/`len`/`is_empty`, `#[must_use]` on the eight pure accessors. Inherent `to_string` removed (shadowed `Display`-driven one); callers continue to use `pq.to_string()` via the `ToString` blanket impl.
- **Rust crate, second hardening pass**: enables `[lints.clippy] pedantic = { level = "warn", priority = -1 }` in `Cargo.toml` and `cargo fmt --check` enforcement. Fixes ~25 lint sites surfaced by `clippy::pedantic` (mostly `doc_markdown` backticks for `O(log_d n)` complexity strings, `uninlined_format_args` updates to `{var}` style, `if !x { ... } else { ... }` branch flips, `i32`/`u32`/`usize` casts replaced with `u32::try_from` / `i32::try_from`, the `pop()` `unwrap` swapped for `?` to satisfy `missing_panics_doc`). One justified `#[allow(clippy::trivially_copy_pass_by_ref)]` in `tests/instrumentation.rs` for a comparator-contract-bound function. `cargo fmt` applied across all source files; one stale rustdoc intra-doc link in `src/instrumentation.rs` rephrased to satisfy `rustdoc::broken_intra_doc_links` under `#![deny(warnings)]`.
- **Rust `README.md` and `src/lib.rs` module-level docs**: new "Comparison-count instrumentation" section with a runnable example using `with_stats` / `InstrumentedPriorityQueue` / `ComparisonStats::{insert,pop,total,reset}`. Core Types table extended to list `StatsCollector`, `NoOpStats`, `ComparisonStats`, `OperationType`, and the `S = NoOpStats` defaulted parameter on `PriorityQueue`. Features bullet expanded.
- **Rust `CONVENTIONS.md`**: Lint Floor section retitled "Active floor" — no longer describes "target floor not yet declared" since the floor is now in place.
- **Rust `Cargo.lock`**: synced to `d-ary-heap` 2.5.0 (was stale at 2.4.0 since the v2.5.0 release).

#### Dijkstra examples (cross-cutting)
- **Time output unified to `"{:.1}µs"`** in all five examples (TypeScript was milliseconds; Go used `time.Duration` auto-format; Rust used `{:?}` on `Duration`; C++ used `µs` / `us` inconsistently). Cross-language wall-time comparison is now apples-to-apples.
- **C++**: literal Unicode `→` and `∞` in output (was raw byte sequences `"\xe2\x86\x92"` and `"inf"`); MSVC build now uses `/utf-8`.
- **C++**: stringly-typed sentinel defaults (`"A"`/`"F"` cleared and re-filled by `parse_args`) replaced with `std::optional<std::string>` resolved once in `main` via `.value_or()`.
- **C++**: TOCTOU-style `fs::exists(p)` then re-open in `resolve_graph_path` collapsed into try-and-fall-through `std::ifstream`. Matches Go/Rust/Zig.
- **C++ `graph_parser`**: integer parsing uses `std::from_chars` instead of `std::stoi(text_.substr(...))` — no per-edge string allocation, no try/catch, no locale.
- **Zig**: replaced the per-string deep-copy loader with `std.json.parseFromSliceLeaky` parsing directly into `Graph` (whose fields already match the JSON schema). Caller owns an `ArenaAllocator`; one `defer arena.deinit()` replaces the previous 8-line per-field cleanup. Eliminated the unused `Edge` import.
- **Rust**: `unwrap_or_default()` on `graph.vertices.{first,last}()` for default endpoints replaced with `.expect("graph has at least one vertex")` — empty graphs now fail loud instead of silently producing `"shortest path from  to "`.
- **Zig**: argument-handling now always-dupes / always-frees `args.graph` (was a fragile string-comparison conditional `defer`; explicit `--graph=small` leaked the duped string).
- **Dijkstra refactor for `--stats` integration**: C++ extracts `dijkstra_with_pq<PQ>(graph, source, pq)` template + thin `dijkstra(g, s, d)` wrapper; Go splits into `Dijkstra` + `DijkstraInstrumented` + private `dijkstraImpl(stats *Stats)`; Zig splits into `dijkstra` + `dijkstraInstrumented` + private generic `runDijkstra(comptime PQ: type, …)`.

#### graphgen
- **`weight_range` validation** lifted from each generator (`erdos_renyi.rs`, `grid.rs`) to a single `Config::validate()` invoked at TOML load time. Single source of truth.

#### Project meta
- **`.gitignore`**: covers v2.6.0 build artifacts (graphgen `target/`, per-language Dijkstra-example build caches), and reserves `test-corpus/` and `experiments/` for paper-artifact paths so future `git add .` is safe.

### Fixed

- **Phase 2 cross-language consistency review** (commit `786f06f`): Go nil-safe `Total()`/`Reset()`; anti-bracketing comments at C++ `pop_many` and Go `PopMany` (delegates to `pop_front`/`Pop`; outer bracket would nest); C++ lockstep pop-order test mirroring Go's; ownership-asymmetry doc added to both file headers.
- **Phase 1 code-review followups** (commit `489996a`): nine fixes from a three-agent review covering cross-language inconsistencies, dead code, and small efficiency wins. See commit body for the full list.

### Findings worth recording

The Phase 2 instrumentation produced one notable cross-language result that is now durable across **all five** language implementations:

- **TypeScript, C++, Go, Rust, and Zig produce byte-for-byte identical comparison counts on `huge_dense`** at every arity:
  - d=2: `insert=2499, pop=43593, increase_priority=16224, total=62316`
  - d=4: `insert=2499, pop=46094, increase_priority=11384, total=59977`
  - d=8: `insert=2499, pop=63710, increase_priority=9727, total=75936`

  Four of the five languages have randomized hash-map iteration order (C++ `unordered_map`, Go `map`, Rust `HashMap`, Zig `StringHashMap`); TypeScript's `Map<K, V>` preserves insertion order. Despite the iteration-order differences, the heap-operation sequence is identical on this graph. The plan predicted divergence; the data says the algorithm is map-iteration-order-insensitive at this seed and corpus.
- **Wall-time gap is therefore entirely time-per-operation**: same operation counts, different cost per heap operation. The `Cost per heap comparison` table in `benchmarks/README.md` makes this concrete (ns-per-comparison at d=8 — median-of-5 ReleaseFast: Go 132 / Zig 157 / C++ 196 / TypeScript 215 / Rust 306). The two AOT-compiled non-GC languages (Go and Zig) cluster together within ~19%; C++ pays a chained-bucket `std::unordered_map` cache-locality cost on MSVC; TypeScript's V8 JIT pays a per-call dispatch tax; Rust pays SipHash + per-call `String` clones in the positions-map lookup. Phase 3 will attribute these with per-section timing inside the heap.

## [2.5.0] - 2026-01-24

### Added

#### Complete Dijkstra Examples (5 Languages)
- **TypeScript**: `examples/dijkstra/TypeScript/` — Working implementation with path reconstruction
- **Go**: `examples/dijkstra/Go/` — Complete implementation using d-heap package
- **Rust**: `examples/dijkstra/Rust/` — Complete implementation using d-ary-heap crate
- **Zig**: `examples/dijkstra/Zig/` — Complete implementation using Zig d-heap module
- **C++**: `examples/dijkstra/Cpp/` — Self-contained implementation (graph data embedded due to no standard JSON support)
- All implementations: identical algorithm, shared test graph (Network Flows Figure 4.7), dynamic vertex sorting
- Performance comparison: tests with d=2, d=4, d=8 heap arities

#### C++ API Completeness
- **C++23 Error Handling**: `std::expected<T, Error>` for safe, expressive error propagation
- **Safe Accessors**: `peek()` returns `std::optional<T>`, `get_position()` returns `std::optional<Position>`
- **Bulk Operations**: `insert_many()` with Floyd's O(n) heapify, `pop_many()` for batch extraction
- **Safe Pop**: `pop_front()` returns `std::optional<T>` for safe extraction
- **Array Access**: `to_array()` for integration with standard library
- **Priority Update Methods**: `update_priority()`, `decrease_priority_by_index()`, `update_priority_by_index()`
- **Safe Variants**: `try_increase_priority()`, `try_decrease_priority()`, `try_update_priority()` returning `std::expected`
- **Factory Functions**: `create()`, `create_with_first()` returning `std::expected<PriorityQueue, Error>`
- **Error Enum**: `InvalidArity`, `ItemNotFound`, `IndexOutOfBounds`, `EmptyQueue`
- **Comprehensive Tests**: 61 tests aligned with Rust test patterns

#### Rust API Completeness
- **Bulk Operations**: `insert_many()` with Floyd's O(n) heapify, `pop_many()` for batch extraction
- **Array Access**: `to_array()` method for integration with Vec and standard library
- **Bidirectional Update**: `update_priority()` for when direction is unknown
- **Index-Based Update**: `update_priority_by_index()` for direct index manipulation
- **Enhanced Tests**: 97 tests (62 comprehensive + 27 doc tests + 8 decrease_priority tests)

#### Go API Completeness
- **Bidirectional Update**: `UpdatePriority()` for when direction is unknown
- **Position Lookup**: `GetPosition()`, `GetPositionByKey()` for O(1) index lookup
- **Index-Based Updates**: `DecreasePriorityByIndex()` for direct index manipulation
- **Snake_case Aliases**: `Update_priority()`, `Decrease_priority_by_index()` for cross-language compatibility
- **Enhanced Tests**: 57 tests covering all functionality

#### Zig API Completeness
- **Bidirectional Update**: `updatePriority()` for when direction is unknown
- **Position Lookup**: `getPosition()` for O(1) index lookup by item identity
- **Index-Based Updates**: `increasePriorityByIndex()`, `decreasePriorityByIndex()` for direct index manipulation
- **Bulk Operations**: `insertMany()` with Floyd's O(n) heapify, `popMany()` for batch extraction
- **Array Access**: `toArray()` for integration with Zig slices
- **Snake_case Aliases**: Full set of aliases (`update_priority()`, `get_position()`, `increase_priority_by_index()`, `decrease_priority_by_index()`, etc.)
- **Enhanced Tests**: 54 tests covering all functionality

#### TypeScript API Alignment
- **Bidirectional Update**: `updatePriority()` for when direction is unknown

### Fixed
- **Zig/TypeScript `decreasePriority()` Semantics**: Now correctly only moves down (was incorrectly calling both directions, causing instrumentation mismatch)

### Changed
- **Cross-Language API Alignment**: All five implementations (C++, Go, Rust, Zig, TypeScript) now have identical core APIs
- **Priority Update Semantics**: Importance-based semantics standardized across all languages
  - `increase_priority()`: Move toward root (more important) — O(log_d n)
  - `decrease_priority()`: Move toward leaves (less important) — O(d × log_d n)
  - `update_priority()`: Both directions when unknown — O((d+1) × log_d n)
- **Documentation**: Synchronized cross-language comments and API references

### Technical Details (C++)
- Requires C++23 for `std::expected` support
- Template specialization maintains full STL compatibility with custom hash/equality functors
- Legacy compatibility methods (`size()`, `empty()`, `getd()`, `put()`) preserved for backward compatibility
- Position-based operations enable O(1) lookup followed by O(log n) update

### Technical Details (Rust)
- `Result<T, Error>` for all fallible operations
- `Display` trait implementation for automatic `format!("{}", pq)` support
- Memory safety with zero-cost abstractions maintained

### Technical Details (Go)
- Idiomatic Go error handling: errors for recoverable conditions, panics for programmer errors
- `Position` type alias used internally for cross-language consistency
- Implements `fmt.Stringer` interface for automatic `fmt.Print()` support

### Technical Details (Zig)
- Error union return types (`!void`, `error.ItemNotFound`, `error.IndexOutOfBounds`)
- Explicit allocator management following Zig best practices
- camelCase as primary convention with snake_case aliases for cross-language compatibility

## [2.4.0] - 2026-01-07

### Added
- **Interactive React Flow Demo**: Live visualization of d-ary heap priority queues with Dijkstra's algorithm
  - Dual-panel layout: heap tree (left) and graph (right)
  - Real-time vertex state coloring (unvisited/in-queue/processed)
  - Arity toggle (d=2, d=4, d=8) for performance comparison
  - Race Mode: Compare all three arities simultaneously
  - Timeline scrubber and playback controls
  - Live at: https://eric-jacopin.github.io/Priority-Queues/
- **TypeScript Instrumentation**: Opt-in comparison counting for performance analysis
  - `instrumentComparator()` for tracking comparisons per operation
  - Theoretical complexity functions for Big-O reference
  - Zero overhead when not used
- **GitHub Actions Deployment**: Automatic demo deployment to GitHub Pages
- **Zig Bulk Operations**: Added `insertMany()` with Floyd's O(n) heapify algorithm for efficient batch insertion
- **Zig Pop Many**: Added `popMany(count)` method for removing multiple highest-priority items at once
- **Zig Array Access**: Added `toArray()` method for integration with Zig slices
- **Zig Snake_case Aliases**: Added `insert_many()`, `pop_many()`, `to_array()` aliases for cross-language consistency
- **Go Snake_case Alias**: Added `Increase_priority_by_index()` alias for consistency with other Go aliases

### Changed
- **Go Internal Consistency**: Now uses `Position` type alias internally instead of raw `int` for documentation clarity
- **Go Documentation**: Renamed `right` to `rightBound` in `bestChildPosition()` with clarifying comments
- **Zig Error Handling**: `swapItems()` now properly propagates errors instead of silently ignoring allocation failures
- **Zig pop() Signature**: Changed from `?T` to `!?T` to properly propagate potential errors from internal operations
- **README Zig Extensions**: Updated to document new bulk operations and array access features

### Technical Details (Demo)
- React 18 + TypeScript + Vite 7
- React Flow (@xyflow/react) with dagre layout
- Uses d-ary-heap NPM package with instrumentation hooks
- Keyboard shortcuts: Space (play/pause), arrows (step), R (reset), 1/2/3 (arity)

### Technical Details (Zig)
- `insertMany()` uses Floyd's heapify algorithm when starting from empty heap (O(n) vs O(n log n))
- `popMany()` returns caller-owned allocated slice that must be freed
- `toArray()` returns a copy of all items in heap order (not priority order)
- Error propagation ensures heap invariants are never silently corrupted

### Technical Details (Go)
- `Position` type alias now used in `swap()`, `bestChildPosition()`, `moveUp()`, `moveDown()`, and `IncreasePriorityByIndex()`
- Variable naming improved for clarity in child position calculations

## [2.3.0] - 2025-12-27

### Added
- **Go Implementation**: Complete d-heap priority queue library in Go with full API parity
- **Go Generics**: Full generic support with `PriorityQueue[T any, K comparable]`
- **Go Dijkstra Example**: Complete working example demonstrating d-heap usage in `examples/dijkstra/Go/`
- **Comparator Utilities**: `MinBy()`, `MaxBy()` factory functions and pre-built comparators for common types
- **Bulk Operations**: `InsertMany()`, `PopMany()` for efficient batch processing with Floyd's heapify algorithm
- **Safe Access**: `Peek()` method returns `(T, bool)` for safe, non-panicking access
- **Go Stringer Interface**: Implements `fmt.Stringer` for automatic `fmt.Print()` support
- **Cross-Language Aliases**: Snake_case method aliases (`Is_empty()`, `Increase_priority()`, etc.) for easy porting
- **Go Workspace**: Added `go.work` file for multi-module development
- **Rust Documentation**: Created comprehensive doctests for all public methods and types
- **Rust Package Metadata**: Enhanced Cargo.toml with complete publication-ready metadata

### Changed
- **Five Language Support**: Project now supports C++, Go, Rust, Zig, and TypeScript
- **Updated Documentation**: Root README.md updated to include Go in all API tables and comparisons
- **Version Information**: Updated to reflect Go implementation as primary feature of v2.3.0
- **Rust Package Naming**: Updated package name to `d-ary-heap` for clarity and consistency
- **Rust Library Naming**: Standardized library name to `d_ary_heap` throughout the codebase

### Technical Details (Go)
- Go module path: `github.com/PCfVW/d-Heap-priority-queue/Go`
- Package name: `dheap`
- Requires Go 1.21+ for generics support
- 47 test cases covering all functionality
- Idiomatic Go error handling: errors for recoverable conditions, panics for programmer errors

## [2.2.0] - 2025-12-26

### Added
- **Examples Infrastructure**: Complete `examples/dijkstra/` directory structure
- **TypeScript Dijkstra Implementation**: Working example with path reconstruction
- **Shared Test Graph**: JSON format graph from Network Flows textbook (Figure 4.7, page 110)
- **Performance Comparisons**: Demonstrates d-ary heap advantages across different arities
- **Algorithm Documentation**: Comprehensive README with complexity analysis

### Enhanced
- **Documentation**: Added detailed algorithm explanation and expected results
- **Visual Diagrams**: Mermaid graph visualization with red edge weights
- **Cross-Language Foundation**: Established pattern for future language implementations

## [2.1.2] - 2025-12-25

### Added
- **Cross-Language Error Handling Documentation**: Comprehensive table showing error handling patterns for each language
- **Return Type Variations Guide**: Detailed documentation of return type differences and safety recommendations
- **Cross-Language Compatibility**: Added `to_string()` alias to Zig implementation for consistency with C++/Rust
- **Language-Specific Error Handling**: Enhanced language extensions documentation with error handling approaches

### Fixed
- **Misleading API Documentation**: Replaced false "unified API" claims with accurate cross-language reference table
- **Method Naming Confusion**: Clarified actual method names per language (camelCase vs snake_case)
- **TypeScript Alias Documentation**: Clear distinction between primary methods and cross-language compatibility aliases
- **Missing Zig Method**: Added documented `to_string()` method alongside existing `toString()`

### Changed
- **Documentation Structure**: Reorganized API documentation to accurately reflect language-specific conventions
- **Error Handling Clarity**: Added best practices and safety recommendations for each language's error handling approach
- **API Reference Format**: Updated from misleading unified table to accurate cross-language mapping

### Technical Details
- Updated main README.md with comprehensive error handling and return type documentation
- Enhanced TypeScript README with clear primary vs alias method documentation
- Added Zig `to_string()` alias method with proper documentation
- Resolved all critical issues identified in comprehensive API audit
- Maintained full backward compatibility across all implementations

## [2.1.1] - 2025-12-18

### Added
- **TypeScript Tooling**: Complete ESLint configuration with TypeScript support
- **Module Type**: Added `"type": "module"` to package.json for better Node.js compatibility
- **Publishing Pipeline**: Complete NPM publishing setup with automated build process

### Fixed
- **ESLint Configuration**: Updated to ESLint v9 flat config format
- **Build Warnings**: Eliminated module type warnings during build process

### Technical Details
- Added ESLint v9 with `@typescript-eslint/eslint-plugin` and `@typescript-eslint/parser`
- Created `eslint.config.js` with modern flat configuration
- Updated package.json with proper module type declaration
- All linting, type checking, and build processes now pass cleanly

## [2.0.0] - 2025-12-18

### Added
- **Zig 0.15.2 Support**: Complete compatibility with latest Zig version
- **Generic Implementation**: Zig implementation now fully generic - use your own item types
- **Comprehensive Test Suite**: 20+ tests covering all functionality and edge cases
- **Module Export**: Zig can now be used as a dependency in other projects via `build.zig.zon`
- **New Methods**: Added `peek()` alias for `front()` and `initCapacity()` for pre-allocation
- **Better Error Handling**: Removed `unreachable` from error paths, proper error propagation
- **Cross-Language API Parity**: All three implementations (C++, Rust, Zig) now have identical APIs

### Changed
- **BREAKING**: Zig API updated for 0.15.2 compatibility (ArrayList, HashMap, format function changes)
- **BREAKING**: Generic type construction now required for custom item types in Zig
- **Improved Documentation**: Updated all READMEs with generic API examples and usage patterns
- **Performance**: Optimized `toString()` method to use writer interface directly

### Technical Details
- Updated ArrayList API for Zig 0.15.2 (now unmanaged, allocator passed to methods)
- Updated HashMap API (managed version with `init(allocator)`)
- Updated build system for new `root_module` and `createModule()` API
- Updated format function signature (`format(self, writer)` instead of 4-argument version)
- Added proper module imports system in `build.zig`

### Migration Guide
For existing Zig users upgrading from v1.x:
- Update to Zig 0.15.2
- Use `DHeapItem` for the built-in Item type (backward compatible)
- For custom types, use the new generic API: `DHeap(T, HashContext(T), Comparator(T))`

## [1.1.0] - 2025-12-17

### Added
- Complete Zig implementation with full API parity
- Unified API method names across all three languages
- `contains()` method for O(1) membership testing
- Comprehensive documentation and examples

### Changed
- Standardized method names for cross-language consistency
- Improved error handling in Rust implementation
- Enhanced documentation across all implementations

## [1.0.0] - 2025-12-16

### Added
- Initial release with C++ and Rust implementations
- Generic d-ary heap with configurable arity
- O(1) item lookup using HashMap/unordered_map
- Min-heap and max-heap support via comparators
- Priority update operations (increase/decrease)
- Comprehensive test suites
- Professional documentation

[2.5.0]: https://github.com/PCfVW/d-Heap-priority-queue/compare/v2.4.0...v2.5.0
[2.4.0]: https://github.com/PCfVW/d-Heap-priority-queue/compare/v2.3.0...v2.4.0
[2.3.0]: https://github.com/PCfVW/d-Heap-priority-queue/compare/v2.2.0...v2.3.0
[2.2.0]: https://github.com/PCfVW/d-Heap-priority-queue/compare/v2.1.2...v2.2.0
[2.1.2]: https://github.com/PCfVW/d-Heap-priority-queue/compare/v2.1.1...v2.1.2
[2.1.1]: https://github.com/PCfVW/d-Heap-priority-queue/compare/v2.0.0...v2.1.1
[2.0.0]: https://github.com/PCfVW/d-Heap-priority-queue/compare/v1.1.0...v2.0.0
[1.1.0]: https://github.com/PCfVW/d-Heap-priority-queue/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/PCfVW/d-Heap-priority-queue/releases/tag/v1.0.0