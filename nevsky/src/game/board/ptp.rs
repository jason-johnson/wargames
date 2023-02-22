#[macro_export]
macro_rules! board {
    (@inner[$graph:ident] locale $name:ident $tuple:expr $(, $($rest:tt)*)?) => {
        let $name = $graph.add_node($tuple);
        board! { @inner[$graph] $($($rest)*)? }
    };
    (@inner[$graph:ident] way ($($args:tt)*), $(, $($rest:tt)*)?) => {
        $graph.add_edge($($args)*);
        board! { @inner[$graph] $($($rest)*)? }
    };
    (@inner[$graph:ident]) => {};
    ($($input:tt)*) => {{
        let mut graph = Graph::new_undirected();
        board! { @inner[graph] $($input)* }
        graph
    }};
}