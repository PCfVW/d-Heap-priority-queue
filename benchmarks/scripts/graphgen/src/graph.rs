use petgraph::graph::Graph;
use petgraph::Directed;

/// Directed weighted graph with string vertex labels.
///
/// Edges are inserted in deterministic order by the generators; iteration via
/// `edge_indices()` and `node_indices()` follows insertion order, which the
/// canonical-output emitter relies on for byte-for-byte reproducibility.
pub type Wg = Graph<String, i64, Directed>;

/// Allocate a graph with `n` vertices labelled "v0", "v1", ..., "v{n-1}".
pub fn empty(n: usize) -> Wg {
    let mut g = Wg::with_capacity(n, 0);
    for i in 0..n {
        g.add_node(format!("v{}", i));
    }
    g
}
