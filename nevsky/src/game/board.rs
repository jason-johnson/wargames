mod hex;
mod ptp;

pub trait Board {
    fn get_x(&self) -> u32;

    fn mutate(&mut self, y: String);
}

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
    ([$graph:ident] way ($src:ident, $dst:ident, $way:expr) $(, $($rest:tt)*)?) => {
        $graph.add_edge($src, $dst, $way);
        $crate::board_inner! { [$graph] $($($rest)*)? }
    };
    ([$graph:ident]) => {};
}

// 