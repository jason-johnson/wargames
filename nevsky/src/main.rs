use clap::Parser;
use petgraph::Graph;
use petgraph::visit::NodeRef;
use crate::parse::Args;
use crate::board::{Locale, Way, PoliticalStatus, Territory};

mod parse;
mod board;

fn main() {
    let args = Args::parse();

    let board = board![
        locale riga (Locale::Bishopric, PoliticalStatus::Teutonic, Territory::Livonia),
        locale wenden (Locale::Castle, PoliticalStatus::Teutonic, Territory::Livonia),
        locale tolowa (Locale::Region, PoliticalStatus::Teutonic, Territory::Livonia),
        way (riga, wenden, Way::Waterway),
    ];

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
