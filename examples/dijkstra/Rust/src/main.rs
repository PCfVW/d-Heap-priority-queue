//! main.rs - Dijkstra's Algorithm Example
//!
//! Demonstrates Dijkstra's shortest path algorithm using d-ary heap priority queues.

mod dijkstra;
mod types;

use clap::Parser;
use dijkstra::{dijkstra, reconstruct_path, INFINITY};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use types::Graph;

#[derive(Parser, Debug)]
#[command(version, about = "Dijkstra's Algorithm Example")]
struct Args {
    /// Graph name (small | medium_sparse | medium_dense | medium_grid | large_sparse | large_dense | large_grid)
    #[arg(long, default_value = "small")]
    graph: String,

    /// Source vertex ID (defaults to "A" for small, "v0" otherwise)
    #[arg(long)]
    source: Option<String>,

    /// Target vertex ID (defaults to "F" for small, last vertex otherwise)
    #[arg(long)]
    target: Option<String>,

    /// Suppress per-vertex distance output
    #[arg(long)]
    quiet: bool,
}

fn load_graph(name: &str) -> Result<Graph, Box<dyn std::error::Error>> {
    let filename = format!("{}.json", name);
    let candidates = [
        PathBuf::from("..").join("graphs").join(&filename),
        PathBuf::from("examples").join("dijkstra").join("graphs").join(&filename),
    ];
    let data = candidates
        .iter()
        .find_map(|p| fs::read_to_string(p).ok())
        .ok_or_else(|| {
            format!(
                "graph file not found for --graph={} (looked in ../graphs/ and examples/dijkstra/graphs/)",
                name
            )
        })?;
    let graph: Graph = serde_json::from_str(&data)?;
    Ok(graph)
}

fn format_results(distances: &std::collections::HashMap<String, i32>, source: &str) {
    println!("Shortest paths from vertex {}:", source);
    println!("================================");

    let mut vertices: Vec<&String> = distances.keys().collect();
    vertices.sort();

    for vertex in vertices {
        let distance = distances.get(vertex).unwrap();
        let distance_str = if *distance == INFINITY {
            "∞".to_string()
        } else {
            distance.to_string()
        };
        println!("{} → {}: {}", source, vertex, distance_str);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let graph = load_graph(&args.graph)?;

    let source = args.source.unwrap_or_else(|| {
        if args.graph == "small" {
            "A".to_string()
        } else {
            graph.vertices.first().cloned().expect("graph has at least one vertex")
        }
    });
    let target = args.target.unwrap_or_else(|| {
        if args.graph == "small" {
            "F".to_string()
        } else {
            graph.vertices.last().cloned().expect("graph has at least one vertex")
        }
    });

    println!("Dijkstra's Algorithm Example");
    if args.graph == "small" {
        println!("Network Flows (Ahuja, Magnanti, Orlin) - Figure 4.7");
    } else {
        println!(
            "graph: {} (|V|={}, |E|={})",
            args.graph,
            graph.vertices.len(),
            graph.edges.len()
        );
    }
    println!("Finding shortest path from {} to {}\n", source, target);

    let arities = vec![2, 4, 8];
    for d in arities {
        println!("--- Using {}-ary heap ---", d);

        let start = Instant::now();
        let result = dijkstra(&graph, &source, d);
        let elapsed = start.elapsed();

        if !args.quiet {
            format_results(&result.distances, &source);
        }

        let path = reconstruct_path(&result.predecessors, &source, &target);
        let path_str = if let Some(p) = &path {
            p.join(" → ")
        } else {
            "No path found".to_string()
        };

        println!("\nShortest path from {} to {}: {}", source, target, path_str);
        if let Some(d) = result.distances.get(&target) {
            println!("Path cost: {}", d);
        }
        let elapsed_us = elapsed.as_secs_f64() * 1_000_000.0;
        println!("Execution time: {:.1}µs\n", elapsed_us);
    }

    Ok(())
}
