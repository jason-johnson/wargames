#[macro_export]
macro_rules! board {
    ($($input:tt)*) => {{
        let mut graph = Graph::new_undirected();
        $crate::board_inner! { [graph] $($input)* }
        graph
    }};
}

#[macro_export]
macro_rules! board_inner {
    ([$graph:ident] locale $name:ident $tuple:expr $(, $($rest:tt)*)?) => {
        let $name = $graph.add_node($tuple);
        $crate::board_inner! { [$graph] $($($rest)*)? }
    };
    ([$graph:ident] way ($($args:tt)*) $(, $($rest:tt)*)?) => {
        $graph.add_edge($($args)*);
        $crate::board_inner! { [$graph] $($($rest)*)? }
    };
    ([$graph:ident]) => {};
}

// ($src:ident, $dst:ident $way:expr)