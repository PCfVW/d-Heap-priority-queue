use crate::config::GraphSpec;
use crate::graph::Wg;
use anyhow::Result;

mod erdos_renyi;
mod grid;

pub fn build(spec: &GraphSpec) -> Result<Wg> {
    match spec {
        GraphSpec::ErdosRenyi(s) => erdos_renyi::generate(s),
        GraphSpec::Grid(s) => grid::generate(s),
    }
}
