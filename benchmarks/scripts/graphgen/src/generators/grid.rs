use crate::config::GridSpec;
use crate::graph::{empty, Wg};
use anyhow::{anyhow, Result};
use petgraph::graph::NodeIndex;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Generate a 4-connected lattice graph. Each lattice connection emits two
/// directed edges (forward and reverse) with the same weight, making the grid
/// effectively undirected for shortest-path purposes.
pub fn generate(spec: &GridSpec) -> Result<Wg> {
    let rows = spec.rows;
    let cols = spec.cols;
    if rows == 0 || cols == 0 {
        return Err(anyhow!("grid dimensions must be > 0, got {}x{}", rows, cols));
    }
    let [w_lo, w_hi] = spec.weight_range;
    if w_lo > w_hi {
        return Err(anyhow!("weight_range invalid: [{}, {}]", w_lo, w_hi));
    }

    let n = rows * cols;
    let mut rng = ChaCha8Rng::seed_from_u64(spec.seed);
    let mut g = empty(n);

    let idx = |r: usize, c: usize| -> usize { r * cols + c };

    // Row-major traversal; emit east and south neighbours of each cell, then
    // the reverse of each. Insertion order is deterministic and pure
    // function of (rows, cols, seed).
    for r in 0..rows {
        for c in 0..cols {
            let here = NodeIndex::new(idx(r, c));
            if c + 1 < cols {
                let east = NodeIndex::new(idx(r, c + 1));
                let w = rng.gen_range(w_lo..=w_hi);
                g.add_edge(here, east, w);
                g.add_edge(east, here, w);
            }
            if r + 1 < rows {
                let south = NodeIndex::new(idx(r + 1, c));
                let w = rng.gen_range(w_lo..=w_hi);
                g.add_edge(here, south, w);
                g.add_edge(south, here, w);
            }
        }
    }

    Ok(g)
}
