use crate::ds::graph::Graph;

pub mod ds;

fn main() {
    let g = Graph::from_edge_list(vec![
        (0, 1),
        (1, 3),
        (1, 4),
        (3, 5),
        (3, 6),
    ], true);
    
    let root = g.root();
    
    eprintln!("{:?}", root)
}
