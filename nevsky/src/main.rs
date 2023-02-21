use clap::Parser;
use petgraph::Graph;
use petgraph::visit::NodeRef;
use crate::parse::Args;
use crate::board::{LocaleInfo, Ways, PoliticalStatus};

mod parse;
mod board;

fn main() {
    let args = Args::parse();

    let mut graph = Graph::new_undirected();
    let riga = graph.add_node(LocaleInfo { name: String::from("Riga"), conquerable: true, value: 2, political_status: PoliticalStatus::Teutonic});
    let wenden = graph.add_node(LocaleInfo { name: String::from("Wenden"), conquerable: true, value: 1, political_status: PoliticalStatus::Teutonic});
    let tolowa = graph.add_node(LocaleInfo { name: String::from("Tolowa"), conquerable: false, value: 0, political_status: PoliticalStatus::Teutonic});
    graph.add_edge(riga, wenden, Ways::Waterway);
    graph.add_edge(wenden, tolowa, Ways::Trackway);

    for neighbor in graph.neighbors(wenden) {
        let i = &graph[neighbor];
        println!("{i:?}");
    }

    println!("{args:?}");
    println!("{graph:?}");
}
