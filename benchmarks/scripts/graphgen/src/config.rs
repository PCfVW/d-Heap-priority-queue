use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub graph: Vec<GraphSpec>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum GraphSpec {
    ErdosRenyi(ErdosRenyiSpec),
    Grid(GridSpec),
}

#[derive(Debug, Deserialize)]
pub struct ErdosRenyiSpec {
    pub name: String,
    pub n: usize,
    /// Total number of directed edges to emit. Must be >= n - 1 to allow
    /// the connectivity-enforcing arborescence; must be <= n * (n - 1)
    /// (the number of distinct directed pairs without self-loops).
    pub target_edges: usize,
    pub seed: u64,
    /// [min, max] inclusive integer weight range.
    pub weight_range: [i64; 2],
}

#[derive(Debug, Deserialize)]
pub struct GridSpec {
    pub name: String,
    pub rows: usize,
    pub cols: usize,
    pub seed: u64,
    pub weight_range: [i64; 2],
}

impl GraphSpec {
    pub fn name(&self) -> &str {
        match self {
            GraphSpec::ErdosRenyi(s) => &s.name,
            GraphSpec::Grid(s) => &s.name,
        }
    }

    fn weight_range(&self) -> [i64; 2] {
        match self {
            GraphSpec::ErdosRenyi(s) => s.weight_range,
            GraphSpec::Grid(s) => s.weight_range,
        }
    }
}

impl Config {
    fn validate(&self) -> Result<()> {
        for spec in &self.graph {
            let [lo, hi] = spec.weight_range();
            if lo > hi {
                return Err(anyhow!(
                    "graph {}: weight_range invalid: [{}, {}]",
                    spec.name(),
                    lo,
                    hi
                ));
            }
        }
        Ok(())
    }
}

pub fn load(path: &Path) -> Result<Config> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading config from {}", path.display()))?;
    let cfg: Config = toml::from_str(&text)
        .with_context(|| format!("parsing config at {}", path.display()))?;
    cfg.validate()?;
    Ok(cfg)
}
