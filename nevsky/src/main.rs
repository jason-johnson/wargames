use clap::Parser;
use petgraph::Graph;
use crate::parse::Args;
use crate::levy_and_campaign::nevsky::{Locale, Way, PoliticalStatus, Territory};

mod parse;
mod game;
mod levy_and_campaign;

fn main() {
    let args = Args::parse();

    let mut graph = Graph::new_undirected();
    let riga = graph.add_node((Locale::Bishopric, PoliticalStatus::Teutonic, Territory::Livonia));
    let wenden = graph.add_node((Locale::Castle, PoliticalStatus::Teutonic, Territory::Livonia));
    let tolowa = graph.add_node((Locale::Region, PoliticalStatus::Teutonic, Territory::Livonia));
    graph.add_edge(riga, wenden, Way::Waterway);
    graph.add_edge(wenden, tolowa, Way::Trackway);

    for neighbor in graph.neighbors(wenden) {
        let i = &graph[neighbor];
        println!("{i:?}");
    }

    println!("{args:?}");
    println!("{graph:?}");
}
