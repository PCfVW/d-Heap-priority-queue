use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod config;
mod emit;
mod generators;
mod graph;
mod verify;

#[derive(Parser)]
#[command(
    name = "graphgen",
    about = "Reproducible graph generator for Dijkstra benchmarks",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate all graphs declared in the config file
    Generate {
        /// Path to graphs.toml
        #[arg(long, default_value = "../../graphs.toml")]
        config: PathBuf,
        /// Output directory for generated JSON files
        #[arg(long, default_value = "../../../examples/dijkstra/graphs")]
        out: PathBuf,
    },
    /// Verify that committed graphs match what the config would generate (byte-for-byte)
    Verify {
        #[arg(long, default_value = "../../graphs.toml")]
        config: PathBuf,
        #[arg(long, default_value = "../../../examples/dijkstra/graphs")]
        graphs: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Generate { config, out } => {
            let cfg = config::load(&config)?;
            for spec in &cfg.graph {
                let g = generators::build(spec)?;
                let path = out.join(format!("{}.json", spec.name()));
                emit::write_canonical(&g, &path)?;
                println!(
                    "wrote {}: |V|={}, |E|={}",
                    path.display(),
                    g.node_count(),
                    g.edge_count()
                );
            }
            Ok(())
        }
        Command::Verify { config, graphs } => {
            let cfg = config::load(&config)?;
            verify::run(&cfg, &graphs)
        }
    }
}
