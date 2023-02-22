use crate::board;
use petgraph::Graph;
use petgraph::Undirected;

#[derive(Debug)]
pub enum PoliticalStatus {
    Russian,
    Teutonic,
}

#[derive(Debug)]
pub enum Way {
    Waterway,
    Trackway,
}

#[derive(Debug)]
pub enum Locale {
    Castle,
    Fort,
    TradeRoute,
    Bishopric,
    City,
    Novgorod,
    Town,
    Region,
}

#[derive(Debug)]
pub enum Territory {
    Livonia,
    Estonia,
    Rus,
}

fn new() -> Graph<(Locale, PoliticalStatus, Territory), Way, Undirected> {
    let result = board![
        locale riga (Locale::Bishopric, PoliticalStatus::Teutonic, Territory::Livonia),
        locale wenden (Locale::Castle, PoliticalStatus::Teutonic, Territory::Livonia),
        locale tolowa (Locale::Region, PoliticalStatus::Teutonic, Territory::Livonia),
        way (riga, wenden, Way::Waterway),
    ];
    result
}