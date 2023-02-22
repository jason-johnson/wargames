use clap::Parser;
use std::ops::Index;
use crate::parse::Args;
use petgraph::stable_graph::NodeIndex;

mod parse;
mod game;
mod levy_and_campaign;

fn main() {
    let args = Args::parse();

    let graph = levy_and_campaign::nevsky::new();
    let wenden = NodeIndex::new(1);
    let x = graph.index(wenden);
    println!("{x:?}");

    for neighbor in graph.neighbors(wenden) {
        let i = &graph[neighbor];
        println!("{i:?}");
    }

    println!("{args:?}");
    println!("{graph:?}");
}
