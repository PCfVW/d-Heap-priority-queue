use crate::graph::Wg;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Serialize a graph to canonical JSON per `examples/dijkstra/graphs/GRAMMAR.md` §2.
///
/// LF line endings, 2-space indent, trailing newline. Vertices on a single
/// line; edges multiline (one per line, 4-space indent).
pub fn to_canonical_string(g: &Wg) -> String {
    let mut out = String::new();
    out.push_str("{\n");

    // "vertices": [id, id, ...]
    out.push_str("  \"vertices\": [");
    for (i, n_idx) in g.node_indices().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push('"');
        out.push_str(&g[n_idx]);
        out.push('"');
    }
    out.push_str("],\n");

    // "edges": [
    //   {"from": "X", "to": "Y", "weight": N},
    //   ...
    // ]
    out.push_str("  \"edges\": [");
    let edge_indices: Vec<_> = g.edge_indices().collect();
    if edge_indices.is_empty() {
        out.push_str("]\n");
    } else {
        out.push('\n');
        for (i, e_idx) in edge_indices.iter().enumerate() {
            let (from, to) = g.edge_endpoints(*e_idx).expect("edge present");
            let weight = g[*e_idx];
            out.push_str("    {\"from\": \"");
            out.push_str(&g[from]);
            out.push_str("\", \"to\": \"");
            out.push_str(&g[to]);
            out.push_str("\", \"weight\": ");
            out.push_str(&weight.to_string());
            out.push('}');
            if i + 1 < edge_indices.len() {
                out.push(',');
            }
            out.push('\n');
        }
        out.push_str("  ]\n");
    }

    out.push_str("}\n");
    out
}

pub fn write_canonical(g: &Wg, path: &Path) -> Result<()> {
    let text = to_canonical_string(g);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating directory {}", parent.display()))?;
    }
    let file = File::create(path)
        .with_context(|| format!("creating file {}", path.display()))?;
    let mut w = BufWriter::new(file);
    w.write_all(text.as_bytes())
        .with_context(|| format!("writing file {}", path.display()))?;
    Ok(())
}
