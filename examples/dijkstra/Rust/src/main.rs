//! main.rs - Dijkstra's Algorithm Example
//!
//! Demonstrates Dijkstra's shortest path algorithm using d-ary heap priority queues.

mod dijkstra;
mod types;

use dijkstra::{dijkstra, reconstruct_path, INFINITY};
use std::fs;
use std::path::Path;
use std::time::Instant;
use types::Graph;

fn load_graph() -> Result<Graph, Box<dyn std::error::Error>> {
    // Try relative path from the example directory
    let graph_path = Path::new("../graphs/small.json");

    let data = if graph_path.exists() {
        fs::read_to_string(graph_path)?
    } else {
        // Try alternative path for when running from project root
        let alt_path = Path::new("examples/dijkstra/graphs/small.json");
        fs::read_to_string(alt_path)?
    };

    let graph: Graph = serde_json::from_str(&data)?;
    Ok(graph)
}

fn format_results(distances: &std::collections::HashMap<String, i32>, source: &str) {
    println!("Shortest paths from vertex {}:", source);
    println!("================================");

    // Sort vertices for consistent output
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
    let graph = load_graph()?;
    let source = "A";
    let target = "F";

    println!("Dijkstra's Algorithm Example");
    println!("Network Flows (Ahuja, Magnanti, Orlin) - Figure 4.7");
    println!("Finding shortest path from {} to {}\n", source, target);

    // Test with different heap arities
    let arities = vec![2, 4, 8];

    for d in arities {
        println!("--- Using {}-ary heap ---", d);

        let start = Instant::now();
        let result = dijkstra(&graph, source, d);
        let elapsed = start.elapsed();

        format_results(&result.distances, source);

        let path = reconstruct_path(&result.predecessors, source, target);
        let path_str = if let Some(p) = &path {
            p.join(" → ")
        } else {
            "No path found".to_string()
        };

        println!("\nShortest path from {} to {}: {}", source, target, path_str);
        println!("Path cost: {}", result.distances.get(target).unwrap());
        println!("Execution time: {:?}\n", elapsed);
    }

    Ok(())
}
