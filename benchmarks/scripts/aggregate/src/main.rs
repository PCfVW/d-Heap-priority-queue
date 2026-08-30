//! aggregate — reads `benchmarks/results/**/*.{jsonl,stats.json,rss.json,profile.json}`,
//! verifies the Phase 2 cross-language comparison-count invariant, and emits
//! `benchmarks/README.md`'s condensed tables, `benchmarks/results/SUMMARY.md`
//! (full 120-cell matrix), and `benchmarks/results/SECTIONS.md` (Phase 3.5
//! per-section attribution). See `benchmarks/methodology.md` for the full
//! measurement contract this tool reports against.

mod collect;
mod report;
mod stats;

use anyhow::{Context, Result};
use std::path::PathBuf;

fn main() -> Result<()> {
    let benchmarks_dir = repo_relative("benchmarks")?;
    let results_dir = benchmarks_dir.join("results");

    let dataset = collect::collect(&results_dir)?;

    collect::check_comparison_invariant(&dataset)
        .context("Phase 2 comparison-count invariant check failed")?;
    println!("comparison-count invariant: OK (all cells agree across 5 languages)");

    let readme_path = benchmarks_dir.join("README.md");
    report::update_readme(&dataset, &readme_path)?;
    println!("wrote {}", readme_path.display());

    let summary_path = results_dir.join("SUMMARY.md");
    report::write_summary(&dataset, &summary_path)?;
    println!("wrote {}", summary_path.display());

    let sections_path = results_dir.join("SECTIONS.md");
    let env_note = "captured 2026-08-30 on the same CPU model as the Phase 3 baseline rig \
        (AMD Ryzen 9 5950X, 64 GB RAM, Windows 11), independently confirmed via `Win32_Processor` \
        rather than assumed from the (older, unregenerated) `env-*.json` files. Run on the default \
        \"Balanced\" power plan in a shared sandbox, not methodology.md's recommended quiesced \
        \"High performance\" setup — absolute numbers may carry more variance than a tuned run; \
        cross-language ratios are the meaningful signal.";
    report::write_sections(&dataset, &sections_path, env_note)?;
    println!("wrote {}", sections_path.display());

    Ok(())
}

/// Resolves `name` relative to the repository root, found by walking up from
/// this binary's crate directory (`benchmarks/scripts/aggregate/`) three
/// levels — the same layout `graphgen` and every `run.ps1` assume.
///
/// # Errors
/// Returns an error if the resolved path does not exist.
fn repo_relative(name: &str) -> Result<PathBuf> {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = crate_dir
        .parent()
        .and_then(std::path::Path::parent)
        .and_then(std::path::Path::parent)
        .context("could not resolve repo root from CARGO_MANIFEST_DIR")?;
    let path = repo_root.join(name);
    if !path.exists() {
        anyhow::bail!("expected path does not exist: {}", path.display());
    }
    Ok(path)
}
