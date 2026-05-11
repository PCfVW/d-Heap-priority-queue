//! main.rs - Dijkstra's Algorithm Example
//!
//! Demonstrates Dijkstra's shortest path algorithm using d-ary heap priority queues.

mod dijkstra;
mod types;

use clap::Parser;
use d_ary_heap::StatsCollector;
use dijkstra::{dijkstra, dijkstra_instrumented, reconstruct_path, INFINITY};
use std::fs;
use std::hint::black_box;
use std::path::PathBuf;
use std::time::Instant;
use types::Graph;

#[derive(Parser, Debug)]
#[command(version, about = "Dijkstra's Algorithm Example")]
struct Args {
    /// Graph name (small | medium_sparse | medium_dense | medium_grid | large_sparse | large_dense | large_grid | huge_dense)
    #[arg(long, default_value = "small")]
    graph: String,

    /// Source vertex ID (defaults to "A" for small, first vertex otherwise)
    #[arg(long)]
    source: Option<String>,

    /// Target vertex ID (defaults to "F" for small, last vertex otherwise)
    #[arg(long)]
    target: Option<String>,

    /// Suppress per-vertex distance output
    #[arg(long)]
    quiet: bool,

    /// Enable comparison-count instrumentation; print per-arity buckets.
    #[arg(long)]
    stats: bool,

    /// Run only one specific arity (default: 2, 4, 8).
    #[arg(long)]
    arity: Option<usize>,

    /// Number of un-timed warmup runs before timed repetitions (--json mode only).
    #[arg(long, default_value_t = 0)]
    warmup: u32,

    /// Number of timed repetitions per arity (--json mode only).
    #[arg(long, default_value_t = 1)]
    repetitions: u32,

    /// Emit JSONL records to stdout instead of human-readable output.
    #[arg(long)]
    json: bool,

    /// Path to env.json; contents are inlined into each wall-time record.
    #[arg(long)]
    env_file: Option<PathBuf>,

    /// Run dijkstra once and emit a single peak-RSS JSON record (Pass 3 of methodology.md).
    /// Requires --arity=<d>. Output is one JSON object on stdout.
    #[arg(long)]
    report_rss: bool,
}

#[cfg(windows)]
fn peak_rss_kb() -> Option<u64> {
    use std::mem;
    use windows_sys::Win32::System::ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
    use windows_sys::Win32::System::Threading::GetCurrentProcess;
    let mut info: PROCESS_MEMORY_COUNTERS = unsafe { mem::zeroed() };
    info.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
    // SAFETY: GetCurrentProcess returns a pseudo-handle that's always valid and needs no Close.
    let ok = unsafe { GetProcessMemoryInfo(GetCurrentProcess(), &mut info, info.cb) };
    if ok != 0 {
        Some(info.PeakWorkingSetSize as u64 / 1024)
    } else {
        None
    }
}

#[cfg(not(windows))]
fn peak_rss_kb() -> Option<u64> {
    // Add /proc/self/status (Linux) or task_info (macOS) when Phase 3 ports off Windows.
    None
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

    let source = args.source.clone().unwrap_or_else(|| {
        if args.graph == "small" {
            "A".to_string()
        } else {
            graph.vertices.first().cloned().expect("graph has at least one vertex")
        }
    });
    let target = args.target.clone().unwrap_or_else(|| {
        if args.graph == "small" {
            "F".to_string()
        } else {
            graph.vertices.last().cloned().expect("graph has at least one vertex")
        }
    });

    let arities: Vec<usize> = match args.arity {
        Some(d) => vec![d],
        None => vec![2, 4, 8],
    };

    let env: Option<serde_json::Value> = match args.env_file.as_ref() {
        Some(p) => Some(serde_json::from_str(&fs::read_to_string(p)?)?),
        None => None,
    };

    if args.report_rss {
        let d = args.arity.ok_or("--report-rss requires --arity=<d>")?;
        // black_box ensures the call (and its allocations) are not elided.
        let _ = black_box(dijkstra(&graph, &source, d));
        let peak = peak_rss_kb().unwrap_or(0);
        let record = serde_json::json!({
            "schema_version": 1,
            "language": "Rust",
            "graph": args.graph,
            "arity": d,
            "peak_rss_kb": peak,
        });
        println!("{}", record);
        return Ok(());
    }

    if args.json {
        for d in arities {
            run_json(&graph, &source, &target, d, &args, env.as_ref());
        }
        return Ok(());
    }

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

    for d in arities {
        println!("--- Using {}-ary heap ---", d);

        let start = Instant::now();
        let (result, stats) = if args.stats {
            let (r, s) = dijkstra_instrumented(&graph, &source, d);
            (r, Some(s))
        } else {
            (dijkstra(&graph, &source, d), None)
        };
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
        if let Some(d_val) = result.distances.get(&target) {
            println!("Path cost: {}", d_val);
        }
        let elapsed_us = elapsed.as_secs_f64() * 1_000_000.0;
        println!("Execution time: {:.1}µs", elapsed_us);

        if let Some(s) = stats {
            println!(
                "Comparison counts: insert={}, pop={}, decrease_priority={}, increase_priority={}, update_priority={}, total={}",
                s.insert(),
                s.pop(),
                s.decrease_priority(),
                s.increase_priority(),
                s.update_priority(),
                s.total()
            );
        }

        println!();
    }

    Ok(())
}

fn run_json(
    graph: &Graph,
    source: &str,
    target: &str,
    d: usize,
    args: &Args,
    env: Option<&serde_json::Value>,
) {
    if args.stats {
        let (_result, stats) = dijkstra_instrumented(graph, source, d);
        let record = serde_json::json!({
            "schema_version": 1,
            "language": "Rust",
            "graph": args.graph,
            "arity": d,
            "comparison_counts": {
                "insert": stats.insert(),
                "pop": stats.pop(),
                "decrease_priority": stats.decrease_priority(),
                "increase_priority": stats.increase_priority(),
                "update_priority": stats.update_priority(),
                "total": stats.total(),
            }
        });
        println!("{}", record);
        return;
    }

    // black_box on both warmup and timed calls — the result is discarded, and
    // we don't want the optimizer to weaken or elide the work between timer reads.
    for _ in 0..args.warmup {
        let _ = black_box(dijkstra(graph, source, d));
    }
    for rep in 1..=args.repetitions {
        let start = Instant::now();
        let _ = black_box(dijkstra(graph, source, d));
        let elapsed = start.elapsed();
        let wall_time_us = elapsed.as_secs_f64() * 1_000_000.0;
        let record = match env {
            Some(env) => serde_json::json!({
                "schema_version": 1,
                "language": "Rust",
                "graph": args.graph,
                "arity": d,
                "source": source,
                "target": target,
                "rep": rep,
                "wall_time_us": wall_time_us,
                "env": env,
            }),
            None => serde_json::json!({
                "schema_version": 1,
                "language": "Rust",
                "graph": args.graph,
                "arity": d,
                "source": source,
                "target": target,
                "rep": rep,
                "wall_time_us": wall_time_us,
            }),
        };
        println!("{}", record);
    }
}
