//! Markdown report generation: `benchmarks/README.md`'s condensed tables,
//! `benchmarks/results/SUMMARY.md`'s full 120-cell matrix, and
//! `benchmarks/results/SECTIONS.md`'s Phase 3.5 headline tables.

use crate::collect::{Dataset, ARITIES};
use crate::stats::compute;
use anyhow::{Context, Result};
use std::fmt::Write as _;
use std::path::Path;

/// Display order used by `benchmarks/README.md`'s existing "Indicative
/// timings" tables and (for continuity) by `SUMMARY.md`: not alphabetical —
/// this is the order the project has published cross-language tables in
/// since v2.6.0.
const README_LANG_ORDER: [(&str, &str); 5] = [
    ("TypeScript", "TypeScript"),
    ("Go", "Go"),
    ("Rust", "Rust"),
    ("Zig", "Zig"),
    ("C++", "Cpp"),
];

/// Display order used by `SECTIONS.md`'s two headline tables — matches
/// `benchmarks/methodology.md`'s "Phase 3.5" section verbatim (it leads with
/// Rust and abbreviates TypeScript to "TS", unlike the README order above).
const SECTIONS_LANG_ORDER: [(&str, &str); 5] = [
    ("Rust", "Rust"),
    ("Go", "Go"),
    ("TS", "TypeScript"),
    ("Zig", "Zig"),
    ("C++", "Cpp"),
];

fn fmt_us(v: f64) -> String {
    format!("{v:.1}")
}

/// Formats one cell's median wall time, in µs, or `n/a` if no data exists
/// for that `(graph, arity, language)` combination.
fn median_cell(ds: &Dataset, graph: &str, arity: u32, lang_key: &str) -> String {
    match ds
        .wall_times
        .get(&(graph.to_string(), arity))
        .and_then(|by_lang| by_lang.get(lang_key))
    {
        Some(samples) if !samples.is_empty() => format!("{} µs", fmt_us(compute(samples).median)),
        _ => "n/a".to_string(),
    }
}

/// Builds one condensed table (methodology.md's "sparse / dense / grid /
/// huge" grouping) over the given graphs, one row per `(graph, arity)`.
fn condensed_table(ds: &Dataset, title: &str, graphs: &[&str]) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "### {title}\n");
    let _ = write!(out, "| graph | arity |");
    for (label, _) in README_LANG_ORDER {
        let _ = write!(out, " {label} |");
    }
    out.push('\n');
    let _ = write!(out, "|---|---:|");
    for _ in README_LANG_ORDER {
        let _ = write!(out, "---:|");
    }
    out.push('\n');

    for &graph in graphs {
        for arity in ARITIES {
            let _ = write!(out, "| `{graph}` | {arity} |");
            for (_, key) in README_LANG_ORDER {
                let _ = write!(out, " {} |", median_cell(ds, graph, arity, key));
            }
            out.push('\n');
        }
    }
    out.push('\n');
    out
}

/// Replaces `benchmarks/README.md`'s wall-time section with four freshly
/// computed Phase 3 condensed tables, and flips the Status table's Phase 3
/// row to Done / adds a Phase 3.5 row.
///
/// Recognizes two possible anchors: the original "## Indicative timings"
/// v2.6.0 ballpark placeholder (first migration), or this tool's own "##
/// Cross-language wall-time (Phase 3)" heading from a prior run (so a future
/// benchmark cycle can re-run this tool to refresh the tables in place,
/// rather than only working once).
///
/// # Errors
/// Returns an error if `readme_path` can't be read/written, or if neither
/// anchor heading is found (the replacement target must exist — silently
/// no-op'ing would leave stale content).
pub fn update_readme(ds: &Dataset, readme_path: &Path) -> Result<()> {
    const OLD_HEADING: &str = "## Indicative timings";
    const CURRENT_HEADING: &str = "## Cross-language wall-time (Phase 3)";

    let original = std::fs::read_to_string(readme_path)
        .with_context(|| format!("reading {}", readme_path.display()))?;

    let (heading, start) = original
        .find(OLD_HEADING)
        .map(|i| (OLD_HEADING, i))
        .or_else(|| original.find(CURRENT_HEADING).map(|i| (CURRENT_HEADING, i)))
        .with_context(|| {
            format!(
                "neither '{OLD_HEADING}' nor '{CURRENT_HEADING}' found in {}",
                readme_path.display()
            )
        })?;
    // The stale placeholder runs up to (but does not include) whichever
    // comes first: the next "## " heading, or the "### Cost per heap
    // comparison" subsection — that one is live Phase 2 content nested
    // *inside* the old "## Indicative timings" section, not part of the
    // placeholder being replaced, so it must survive.
    let search_from = start + heading.len();
    let next_h2 = original[search_from..].find("\n## ");
    let cost_per_cmp = original[search_from..].find("\n### Cost per heap comparison");
    let end = match (next_h2, cost_per_cmp) {
        (Some(a), Some(b)) => search_from + a.min(b) + 1,
        (Some(a), None) => search_from + a + 1,
        (None, Some(b)) => search_from + b + 1,
        (None, None) => original.len(),
    };

    let mut section = String::new();
    let _ = writeln!(section, "## Cross-language wall-time (Phase 3)\n");
    let _ = writeln!(
        section,
        "Median of N=10 timed repetitions (K=2 warmup discarded) per `(language, graph, arity)` cell, \
         captured per the protocol in [`methodology.md`](methodology.md). Full 120-cell matrix with \
         IQR/min/max in [`results/SUMMARY.md`](results/SUMMARY.md); per-section attribution (why the \
         languages differ) in [`results/SECTIONS.md`](results/SECTIONS.md). `small` is excluded here — \
         see methodology.md's sample-matrix note on why it sits at the edge of timer resolution.\n"
    );
    section.push_str(&condensed_table(
        ds,
        "Sparse graphs",
        &["medium_sparse", "large_sparse"],
    ));
    section.push_str(&condensed_table(
        ds,
        "Dense graphs",
        &["medium_dense", "large_dense"],
    ));
    section.push_str(&condensed_table(
        ds,
        "Grid graphs",
        &["medium_grid", "large_grid"],
    ));
    section.push_str(&condensed_table(ds, "`huge_dense`", &["huge_dense"]));

    let mut updated = String::with_capacity(original.len());
    updated.push_str(&original[..start]);
    updated.push_str(&section);
    updated.push_str(&original[end..]);

    // Flip the Status table's Phase 3 row and add a Phase 3.5 row, if not
    // already present (idempotent re-runs of this tool shouldn't duplicate it).
    let old_row = "| 3 | Benchmark runners + results | 🚧 Planned (depends on Phase 2) |";
    let new_rows = "| 3 | Benchmark runners + results | ✅ Done (v2.7.0) |\n\
                    | 3.5 | Per-section timing attribution | ✅ Done (v2.7.0) |";
    let updated = if updated.contains(old_row) {
        updated.replace(old_row, new_rows)
    } else {
        updated
    };

    std::fs::write(readme_path, updated)
        .with_context(|| format!("writing {}", readme_path.display()))?;
    Ok(())
}

/// Writes `benchmarks/results/SUMMARY.md`: the full 120-cell wall-time
/// matrix (all 8 graphs, including `small`), one table per graph, each cell
/// showing `median (IQR) [min–max]` in µs.
///
/// # Errors
/// Returns an error if the file can't be written.
pub fn write_summary(ds: &Dataset, out_path: &Path) -> Result<()> {
    use crate::collect::GRAPHS;

    let mut out = String::new();
    let _ = writeln!(
        out,
        "# Phase 3 Full Result Matrix\n\n\
         Generated by `benchmarks/scripts/aggregate/`. All 8 graphs × 3 arities × 5 languages \
         (120 cells). Each cell is `median (IQR) [min–max]` in µs over the N=10 timed repetitions \
         — see [`methodology.md`](../methodology.md) for the protocol. `n/a` means that cell's \
         result files were not found.\n"
    );

    for &graph in &GRAPHS {
        let _ = writeln!(out, "## `{graph}`\n");
        let _ = write!(out, "| arity |");
        for (label, _) in README_LANG_ORDER {
            let _ = write!(out, " {label} |");
        }
        out.push('\n');
        let _ = write!(out, "|---:|");
        for _ in README_LANG_ORDER {
            let _ = write!(out, "---|");
        }
        out.push('\n');

        for arity in ARITIES {
            let _ = write!(out, "| {arity} |");
            for (_, key) in README_LANG_ORDER {
                let cell = match ds
                    .wall_times
                    .get(&(graph.to_string(), arity))
                    .and_then(|by_lang| by_lang.get(key))
                {
                    Some(samples) if !samples.is_empty() => {
                        let s = compute(samples);
                        format!(
                            "{} ({}) [{}–{}]",
                            fmt_us(s.median),
                            fmt_us(s.iqr),
                            fmt_us(s.min),
                            fmt_us(s.max)
                        )
                    }
                    _ => "n/a".to_string(),
                };
                let _ = write!(out, " {cell} |");
            }
            out.push('\n');
        }
        out.push('\n');
    }

    std::fs::write(out_path, out).with_context(|| format!("writing {}", out_path.display()))?;
    Ok(())
}

/// A named row in a [`write_section_table`] call: its label, and the
/// accessor that pulls that section's value out of a [`SectionUs`](crate::collect::SectionUs).
type SectionRow = (&'static str, fn(&crate::collect::SectionUs) -> f64);

/// Appends one `SECTIONS.md` table (shared by the percentage-of-total and
/// absolute-µs tables, which differ only in heading, column-header suffix,
/// row order, and how a raw section value is formatted).
fn write_section_table(
    out: &mut String,
    heading: &str,
    col_suffix: &str,
    by_lang: &std::collections::BTreeMap<String, crate::collect::SectionUs>,
    rows: &[SectionRow],
    format_value: impl Fn(f64, &crate::collect::SectionUs) -> String,
) {
    let _ = writeln!(out, "{heading}");
    let _ = write!(out, "| Section |");
    for (label, _) in SECTIONS_LANG_ORDER {
        let _ = write!(out, " {label}{col_suffix} |");
    }
    out.push('\n');
    let _ = write!(out, "|---|");
    for _ in SECTIONS_LANG_ORDER {
        let _ = write!(out, "---:|");
    }
    out.push('\n');
    for (row_label, accessor) in rows {
        let _ = write!(out, "| {row_label} |");
        for (_, lang_key) in SECTIONS_LANG_ORDER {
            let section = &by_lang[lang_key];
            let _ = write!(out, " {} |", format_value(accessor(section), section));
        }
        out.push('\n');
    }
    out.push('\n');
}

/// Reduces N repeated `--profile-sections` samples to one [`SectionUs`] whose
/// five fields are each the **median** of that field across the samples,
/// computed independently via [`compute`] — matching how Phase 3's wall-time
/// median is computed, and for the same reason (median is robust to a single
/// cold/jittery rep; see `methodology.md`'s "Reported summary statistics").
///
/// # Panics
/// Panics if `samples` is empty — callers only invoke this after confirming
/// the language has at least one recorded rep.
fn median_section(samples: &[crate::collect::SectionUs]) -> crate::collect::SectionUs {
    let field = |get: fn(&crate::collect::SectionUs) -> f64| {
        let values: Vec<f64> = samples.iter().map(get).collect();
        compute(&values).median
    };
    crate::collect::SectionUs {
        setup: field(|s| s.setup),
        pop: field(|s| s.pop),
        relax: field(|s| s.relax),
        inc_pri: field(|s| s.inc_pri),
        total: field(|s| s.total),
    }
}

/// Appends `SECTIONS.md`'s "## Timer call overhead" section: one row per
/// language, sorted cheapest-first, with a ratio column normalized to the
/// cheapest (1.00×) — directly answers "could Phase 3.5's cross-language
/// section differences be explained by differing instrumentation cost
/// rather than real algorithmic differences?"
///
/// # Errors
/// Returns an error if any of the five languages has no `--timer-overhead`
/// data collected.
fn write_timer_overhead(out: &mut String, ds: &Dataset) -> Result<()> {
    let mut rows: Vec<(&str, crate::stats::CellStats)> = Vec::with_capacity(crate::collect::LANGUAGES.len());
    for lang in crate::collect::LANGUAGES {
        let samples = ds
            .timer_overhead_us
            .get(lang)
            .with_context(|| format!("no --timer-overhead data found for {lang}"))?;
        rows.push((lang, compute(samples)));
    }
    rows.sort_by(|a, b| a.1.median.partial_cmp(&b.1.median).expect("per_call_us is never NaN"));
    let cheapest_us = rows[0].1.median;

    let _ = writeln!(
        out,
        "## Timer call overhead (instrument floor)\n\n\
         Median per-call cost of this process's own native monotonic-timer read, from 10 \
         independent batches of 1,000,000 back-to-back `--timer-overhead` calls. Every Phase 3.5 \
         section pays roughly one \"bracket\" (2 calls: start + elapsed) per section per main-loop \
         iteration, so this is the floor beneath every number in the tables above — it answers \
         whether the cross-language section differences could be an artifact of differing \
         instrumentation cost rather than real algorithmic differences.\n"
    );
    let _ = writeln!(out, "| Language | Median per-call (ns) | Ratio | Median per-bracket (ns) | IQR (ns) |");
    let _ = writeln!(out, "|---|---:|---:|---:|---:|");
    for (lang, stats) in &rows {
        let ratio = stats.median / cheapest_us;
        let _ = writeln!(
            out,
            "| {lang} | {:.1} | {:.2}× | {:.1} | {:.1} |",
            stats.median * 1000.0,
            ratio,
            stats.median * 1000.0 * 2.0,
            stats.iqr * 1000.0,
        );
    }

    let fastest = rows[0].0;
    let slowest = &rows[rows.len() - 1];
    let _ = writeln!(
        out,
        "\n{fastest}'s timer read is the cheapest of the five; {}'s is {:.1}× more expensive per \
         call. On `huge_dense × d=8` the main loop pays on the order of 10⁵ brackets total across \
         `setup`/`pop`/`relax`/`inc_pri` combined (`pop`/`relax` alone are bounded by \\|V\\|=2500 and \
         \\|E\\|=125000 respectively), so even this small a per-call difference compounds to a real, \
         non-zero, systematic offset between languages — but at nanoseconds per call against the \
         Finding's microseconds-to-milliseconds section totals, it is one to two orders of magnitude \
         too small to be the primary explanation for the Rust-vs-Go gap on its own. It is a genuine \
         confound this document does not correct for.\n",
        slowest.0,
        slowest.1.median / cheapest_us,
    );

    Ok(())
}

/// Appends `SECTIONS.md`'s "## Call-count-based overhead attribution"
/// section: uses the plain (non-timer) `pop_count`/`relax_count` counters —
/// verified identical across all 10 reps and all 5 languages by
/// [`crate::collect::check_call_count_invariant`] — alongside the
/// `--timer-overhead` medians to test, with measured numbers instead of the
/// `~|V|`/`~|E|` guess, how much of the Rust-vs-Go `pop`/`relax` gap a naive
/// "per-bracket timer cost × bracket count" model would predict.
///
/// # Errors
/// Returns an error if `huge_dense × d=8` call-count or timer-overhead data
/// is missing for Rust or Go.
fn write_overhead_attribution(
    out: &mut String,
    ds: &Dataset,
    by_lang: &std::collections::BTreeMap<String, crate::collect::SectionUs>,
) -> Result<()> {
    let key = ("huge_dense".to_string(), 8u32);
    let (pop_count, relax_count) = ds
        .call_counts
        .get(&key)
        .and_then(|by_lang| by_lang.get("Rust"))
        .and_then(|samples| samples.first())
        .copied()
        .context("no Phase 3.5 call-count data found for Rust on huge_dense × d=8")?;

    let rust_per_call_us = ds
        .timer_overhead_us
        .get("Rust")
        .map(|s| compute(s).median)
        .context("no --timer-overhead data for Rust")?;
    let go_per_call_us = ds
        .timer_overhead_us
        .get("Go")
        .map(|s| compute(s).median)
        .context("no --timer-overhead data for Go")?;
    let per_bracket_diff_us = (rust_per_call_us - go_per_call_us) * 2.0;

    // Call counts are bounded by this project's graph corpus (≤125,000, `huge_dense`'s edge
    // count) — nowhere near f64's 52-bit mantissa limit, so the precision-loss lint is a false
    // positive here.
    #[allow(clippy::cast_precision_loss)]
    let predicted_us = |count: u64| per_bracket_diff_us * count as f64;
    let pct_of_gap = |predicted: f64, actual: f64| {
        if actual > 0.0 {
            format!("{:.0}%", predicted / actual * 100.0)
        } else {
            "n/a".to_string()
        }
    };

    let rust = &by_lang["Rust"];
    let go = &by_lang["Go"];
    let pop_gap_us = rust.pop - go.pop;
    let relax_gap_us = rust.relax - go.relax;
    let pop_predicted_us = predicted_us(pop_count);
    let relax_predicted_us = predicted_us(relax_count);
    let pop_pct_str = pct_of_gap(pop_predicted_us, pop_gap_us);
    let relax_pct_str = pct_of_gap(relax_predicted_us, relax_gap_us);

    let _ = writeln!(
        out,
        "## Call-count-based overhead attribution\n\n\
         `pop_count`/`relax_count` are plain counters (no timer) recorded alongside every \
         `--profile-sections` rep specifically to check the `~|V|`/`~|E|` guess used when relating \
         the timer-overhead table above to the `pop`/`relax` gap — confirmed exact and identical \
         across all 10 reps and all 5 languages: `pop_count`={pop_count}, `relax_count`={relax_count} \
         on `huge_dense × d=8`. The table below asks: if the entire Rust-vs-Go gap in a section came \
         from Rust's timer read costing {:.1} ns more per bracket than Go's (from the table above), \
         how much of the *measured* gap would that predict?\n",
        per_bracket_diff_us * 1000.0,
    );
    let _ = writeln!(out, "| Section | Bracket count | Predicted overhead-only gap (µs) | Actual measured gap (µs) | Overhead as % of gap |");
    let _ = writeln!(out, "|---|---:|---:|---:|---:|");
    let _ = writeln!(
        out,
        "| pop | {pop_count} | {} | {} | {} |",
        fmt_us(pop_predicted_us),
        fmt_us(pop_gap_us),
        pct_of_gap(pop_predicted_us, pop_gap_us)
    );
    let _ = writeln!(
        out,
        "| relax | {relax_count} | {} | {} | {} |",
        fmt_us(relax_predicted_us),
        fmt_us(relax_gap_us),
        pct_of_gap(relax_predicted_us, relax_gap_us)
    );

    let pop_sentence = if pop_gap_us > 0.0 {
        format!(
            "For `pop`'s ~2,500 brackets, the instrumentation asymmetry alone predicts {pop_pct_str} \
             of the measured gap."
        )
    } else {
        format!(
            "For `pop`, Go's own median was {} µs slower than Rust's on this run — a negative gap \
             the {}-µs predicted instrumentation asymmetry can't be sensibly compared against; at \
             ~2,500 brackets the effect is small enough that run-to-run noise can flip its sign \
             (see the measurement-environment note above)."
        , fmt_us(-pop_gap_us), fmt_us(pop_predicted_us))
    };
    let relax_sentence = if relax_gap_us > 0.0 {
        format!(
            "For `relax`'s ~125,000 brackets, the same per-bracket difference, extrapolated \
             linearly, predicts {relax_pct_str} of the measured gap — {} it, which means the \
             isolated, back-to-back `--timer-overhead` benchmark does not transfer linearly to a \
             timer call interleaved with real hashmap/heap work at this call volume.",
            if relax_predicted_us > relax_gap_us { "overshooting" } else { "undershooting" }
        )
    } else {
        format!(
            "For `relax`, Go's own median was {} µs slower than Rust's on this run, so the \
             instrumentation-asymmetry model has no positive gap to attribute against.",
            fmt_us(-relax_gap_us)
        )
    };

    let _ = writeln!(
        out,
        "\n{pop_sentence} {relax_sentence} Whatever is producing either section's actual gap on a \
         given run, this linear model cannot cleanly subtract out the instrumentation confound, and \
         the hashmap-cost hypothesis is neither confirmed nor ruled out by this table alone.\n"
    );

    Ok(())
}

/// Writes `benchmarks/results/SECTIONS.md`: the Phase 3.5 headline tables at
/// `huge_dense × d=8` (percentage of total time per section, and absolute
/// µs per section), plus a short honest note on the measurement environment
/// and the finding itself, computed from the real data — not assumed from
/// the hypothesis in methodology.md.
///
/// Each language's numbers are the **median of N=10 timed repetitions**
/// (K=2 warmup discarded), the same protocol Phase 3's wall-time table uses
/// — a revision from Phase 3.5's original single-un-iterated-call design,
/// which the first real run showed was too noisy to attribute cross-language
/// gaps (see `CHANGELOG.md`).
///
/// # Errors
/// Returns an error if `huge_dense × d=8` section data is missing for any
/// language, or if the file can't be written.
pub fn write_sections(ds: &Dataset, out_path: &Path, env_note: &str) -> Result<()> {
    let key = ("huge_dense".to_string(), 8u32);
    let samples_by_lang = ds
        .sections
        .get(&key)
        .context("no Phase 3.5 section data found for huge_dense × d=8")?;

    for (label, lang_key) in SECTIONS_LANG_ORDER {
        if !samples_by_lang.contains_key(lang_key) {
            anyhow::bail!("missing huge_dense × d=8 section data for {label} ({lang_key})");
        }
    }

    let by_lang: std::collections::BTreeMap<String, crate::collect::SectionUs> = samples_by_lang
        .iter()
        .map(|(lang, samples)| (lang.clone(), median_section(samples)))
        .collect();
    let by_lang = &by_lang;
    let rep_count = samples_by_lang["Rust"].len();

    let mut out = String::new();
    let _ = writeln!(
        out,
        "# Phase 3.5 — Per-Section Timing Attribution\n\n\
         Generated by `benchmarks/scripts/aggregate/` from {rep_count} timed `--profile-sections` \
         repetitions (K=2 warmup discarded) per `(language, graph, arity)` cell — the same protocol \
         Phase 3's wall-time table uses, reused here after the original single-un-iterated-call \
         design proved too noisy to attribute cross-language gaps (see `CHANGELOG.md`). Each number \
         below is the **median** across those repetitions. See \
         [`methodology.md`](../methodology.md#phase-35-per-section-timing) for the mechanism and \
         the cross-language section-boundary contract. All 120 `.profile.jsonl` cells (8 graphs × 3 \
         arities × 5 languages) are on disk in `benchmarks/results/<Language>/`; the tables below \
         focus on the headline `huge_dense × d=8` cell per methodology.md.\n\n\
         **Measurement environment:** {env_note}\n"
    );

    // Percentage-of-total table: setup, pop, relax, inc_pri (methodology.md's row order).
    write_section_table(
        &mut out,
        "## Percentage of total time per section\n",
        " %",
        by_lang,
        &[
            ("setup", |s| s.setup),
            ("pop", |s| s.pop),
            ("relax", |s| s.relax),
            ("inc_pri", |s| s.inc_pri),
        ],
        |value, section| {
            if section.total > 0.0 {
                format!("{:.1}%", value / section.total * 100.0)
            } else {
                "0.0%".to_string()
            }
        },
    );

    // Absolute-µs table: relax, pop, inc_pri, setup (methodology.md's row order —
    // leads with relax, the expected discriminator).
    write_section_table(
        &mut out,
        "## Absolute time per section (µs) at `huge_dense × d=8`\n",
        "",
        by_lang,
        &[
            ("relax", |s| s.relax),
            ("pop", |s| s.pop),
            ("inc_pri", |s| s.inc_pri),
            ("setup", |s| s.setup),
        ],
        |value, _section| fmt_us(value),
    );

    write_finding(&mut out, by_lang, samples_by_lang, rep_count);
    write_timer_overhead(&mut out, ds)?;
    write_overhead_attribution(&mut out, ds, by_lang)?;

    std::fs::write(out_path, out).with_context(|| format!("writing {}", out_path.display()))?;
    Ok(())
}

/// Appends `SECTIONS.md`'s "## Finding" section: the headline Rust-vs-Go
/// comparison, computed from the real median data (not assumed from the
/// hypothesis), plus the IQR-based answer to "did repeated sampling actually
/// help?" and the caveats that still apply after it.
fn write_finding(
    out: &mut String,
    by_lang: &std::collections::BTreeMap<String, crate::collect::SectionUs>,
    samples_by_lang: &std::collections::BTreeMap<String, Vec<crate::collect::SectionUs>>,
    rep_count: usize,
) {
    let rust = &by_lang["Rust"];
    let go = &by_lang["Go"];
    let relax_ratio = if go.relax > 0.0 { rust.relax / go.relax } else { f64::INFINITY };
    let pop_ratio = if go.pop > 0.0 { rust.pop / go.pop } else { f64::INFINITY };
    let total_gap_us = rust.total - go.total;

    // IQR of `total` per language, as a direct answer to "did repeating the
    // call actually tighten the signal vs. the original single-sample run?"
    let total_iqr = |lang: &str| {
        let samples: Vec<f64> = samples_by_lang[lang].iter().map(|s| s.total).collect();
        compute(&samples).iqr
    };

    let _ = writeln!(out, "## Finding\n");
    let _ = writeln!(
        out,
        "On `huge_dense × d=8`, median-of-{rep_count} Rust total ({} µs) exceeds Go's ({} µs) by \
         {} µs. `relax` is {:.2}× Go's on Rust, versus {:.2}× on `pop` — {}",
        fmt_us(rust.total),
        fmt_us(go.total),
        fmt_us(total_gap_us),
        relax_ratio,
        pop_ratio,
        if relax_ratio > pop_ratio * 1.2 {
            "consistent with the hashmap-in-`relax` hypothesis (SipHash + `String` clones) from \
             methodology.md: the gap concentrates in `relax`, not the heap-only sections."
        } else {
            "NOT clearly concentrated in `relax` relative to the heap-only sections even after \
             repeated sampling — the hashmap hypothesis is not confirmed by this data alone; see \
             the caveats below before drawing conclusions."
        }
    );
    let _ = writeln!(
        out,
        "\nRepeated sampling (K=2 warmup / N={rep_count}) was adopted specifically because Phase \
         3.5's original single-call design produced a result (Rust's heap-only `pop` showing a \
         *larger* relative gap than the hashmap-heavy `relax`) that looked like noise, not signal. \
         Total-time IQR across the {rep_count} reps: Rust {} µs, Go {} µs — {}.",
        fmt_us(total_iqr("Rust")),
        fmt_us(total_iqr("Go")),
        if total_iqr("Rust") / rust.total < 0.15 && total_iqr("Go") / go.total < 0.15 {
            "both comfortably under 15% of their own median, so the medians above are a reasonably \
             stable summary of this machine at this point in time"
        } else {
            "at least one language's spread is not small relative to its median, so treat the \
             medians above as directionally useful rather than tightly pinned down"
        }
    );
    let _ = writeln!(
        out,
        "\n**Caveats specific to this run:**\n\
         - Go's `time.Since` on Windows quantizes to a coarse grid (documented in methodology.md's \
           Phase 3 timer-resolution caveat); individual `contains`/`increase_priority` calls are \
           often faster than that resolution, so Go's `inc_pri` (and occasionally other sections) \
           can under-report on any given rep — a measurement floor, not zero real cost. The median \
           across {rep_count} reps reduces but does not eliminate this.\n\
         - Phase 3.5 timer overhead (~50–200 ns per main-loop iteration, per methodology.md) is \
           symmetric across languages but not subtracted from any number above, and repeated \
           sampling doesn't change that — it reduces cross-rep noise, not the overhead itself.\n\
         - Section-timing medians are still not directly comparable to `results/SUMMARY.md`'s \
           `huge_dense` wall-time medians — the section timers themselves add overhead the plain \
           wall-time measurement doesn't pay.\n"
    );
}
