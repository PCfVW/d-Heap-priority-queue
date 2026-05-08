use crate::config::ErdosRenyiSpec;
use crate::graph::{empty, Wg};
use anyhow::{anyhow, Result};
use petgraph::graph::NodeIndex;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

/// Generate a directed Erdős–Rényi-style graph with connectivity guarantee.
///
/// Algorithm:
/// 1. Build a random arborescence rooted at `v0` (n-1 directed edges, each
///    non-root vertex picks a uniformly-random parent from earlier indices).
///    This guarantees every vertex is reachable from `v0`.
/// 2. Sample remaining `target_edges - (n-1)` additional directed edges
///    uniformly without replacement from the set of distinct ordered pairs
///    not already chosen.
///
/// Edge weights are sampled uniformly from `[weight_range[0], weight_range[1]]`.
pub fn generate(spec: &ErdosRenyiSpec) -> Result<Wg> {
    let n = spec.n;
    if n < 2 {
        return Err(anyhow!("ER graph requires n >= 2, got {}", n));
    }
    if spec.target_edges < n - 1 {
        return Err(anyhow!(
            "target_edges {} too low for connectivity (n={} requires >= {})",
            spec.target_edges,
            n,
            n - 1
        ));
    }
    let max_edges = n * (n - 1);
    if spec.target_edges > max_edges {
        return Err(anyhow!(
            "target_edges {} exceeds maximum {} directed edges for n={}",
            spec.target_edges,
            max_edges,
            n
        ));
    }
    let [w_lo, w_hi] = spec.weight_range;

    let mut rng = ChaCha8Rng::seed_from_u64(spec.seed);
    let mut g = empty(n);
    let mut edge_set: HashSet<(usize, usize)> = HashSet::new();

    // Step 1: arborescence rooted at v0. Every non-root vertex i picks a
    // uniformly-random parent from {0..i}, guaranteeing v0 reaches all.
    for i in 1..n {
        let parent = rng.gen_range(0..i);
        let weight = rng.gen_range(w_lo..=w_hi);
        g.add_edge(NodeIndex::new(parent), NodeIndex::new(i), weight);
        edge_set.insert((parent, i));
    }

    // Step 2: sample remaining directed edges via rejection sampling.
    let remaining = spec.target_edges - (n - 1);
    let mut added = 0;
    while added < remaining {
        let from = rng.gen_range(0..n);
        let to = rng.gen_range(0..n);
        if from == to {
            continue;
        }
        if !edge_set.insert((from, to)) {
            continue;
        }
        let weight = rng.gen_range(w_lo..=w_hi);
        g.add_edge(NodeIndex::new(from), NodeIndex::new(to), weight);
        added += 1;
    }

    Ok(g)
}
