use crate::config::Config;
use crate::{emit, generators};
use anyhow::{anyhow, Result};
use std::path::Path;

/// Re-emit each graph from the config and compare byte-for-byte against the
/// committed file in `graphs_dir`. Returns Err on any drift.
pub fn run(cfg: &Config, graphs_dir: &Path) -> Result<()> {
    let mut all_ok = true;
    for spec in &cfg.graph {
        let g = generators::build(spec)?;
        let expected = emit::to_canonical_string(&g);
        let path = graphs_dir.join(format!("{}.json", spec.name()));
        let actual = std::fs::read_to_string(&path)
            .map_err(|e| anyhow!("reading {}: {}", path.display(), e))?;
        if expected != actual {
            eprintln!("DRIFT: {}", path.display());
            // Locate the first differing line for diagnostics.
            for (i, (a, b)) in actual.lines().zip(expected.lines()).enumerate() {
                if a != b {
                    eprintln!("  first diff at line {}:", i + 1);
                    eprintln!("    actual:   {}", a);
                    eprintln!("    expected: {}", b);
                    break;
                }
            }
            all_ok = false;
        } else {
            println!("OK: {}", path.display());
        }
    }
    if !all_ok {
        return Err(anyhow!("one or more graphs drifted from spec"));
    }
    Ok(())
}
