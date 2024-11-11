use crate::ds::graph::Graph;

pub mod ds;
mod opt_block;

fn main() {
    let g = Graph::from_edge_list(vec![
        (0, 1),
        (1, 3),
        (1, 4),
        (3, 5),
        (3, 6),
    ], true);
    let f = Graph::from_edge_list(vec![
        (1, 2),
        (2, 4),
        (2, 5),
        (4, 6),
        (4, 7),
    ], true);
    
    println!("{:?}", g.is_isomorphic_to(&f));
}
