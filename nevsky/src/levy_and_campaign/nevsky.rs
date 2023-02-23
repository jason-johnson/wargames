use crate::board;
use petgraph::Graph;
use petgraph::Undirected;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scenario {
    PleskauQuickStart,
    Pleskau,
    Watland,
    ReturnOfThePrince,
    ReturnOfThePrinceNicolle,
    Peipus,
    CrusadeOnNovgorod,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PoliticalStatus {
    Russian,
    Teutonic,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Way {
    Waterway,
    Trackway,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Territory {
    Livonia,
    Estonia,
    Rus,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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