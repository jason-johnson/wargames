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

#[derive(Debug)]
pub enum Region {
  Riga,
  Wenden,
  Tolowa,
  Adsel,
}

pub fn new() -> Graph<(Region, Locale, PoliticalStatus, Territory), Way, Undirected> {
    let result = board![
        locale riga (Region::Riga, Locale::Bishopric, PoliticalStatus::Teutonic, Territory::Livonia),
        locale wenden (Region::Wenden, Locale::Castle, PoliticalStatus::Teutonic, Territory::Livonia),
        locale tolowa (Region::Tolowa, Locale::Region, PoliticalStatus::Teutonic, Territory::Livonia),
        way (riga, wenden, Way::Waterway),
        way (wenden, tolowa, Way::Trackway)
    ];
    result
}