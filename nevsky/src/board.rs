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

#[macro_export]
macro_rules! board {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_graph = Graph::new_undirected();
            $(
                temp_graph.add_node($x);
            )*
            temp_graph
        }
    };
}