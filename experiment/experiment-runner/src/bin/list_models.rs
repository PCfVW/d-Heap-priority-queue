//! List available Anthropic models
//!
//! Usage:
//!   cargo run --bin list_models
//!   cargo run --bin list_models -- --json

use anyhow::Result;
use clap::Parser;

// We need to reference the main crate
use experiment_runner::anthropic::AnthropicProvider;

#[derive(Parser, Debug)]
#[command(name = "list_models")]
#[command(about = "List available Anthropic models")]
struct Args {
    /// Output as JSON
    #[arg(long)]
    json: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let provider = AnthropicProvider::new()?;
    let models = provider.list_models().await?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&models)?);
    } else {
        println!("Available Anthropic Models (newest first):");
        println!("{}", "=".repeat(70));
        println!("{:<45} {}", "Model ID", "Display Name");
        println!("{}", "-".repeat(70));
        for model in &models {
            println!("{:<45} {}", model.id, model.display_name);
        }
        println!("{}", "=".repeat(70));
        println!("Total: {} models", models.len());
    }

    Ok(())
}
