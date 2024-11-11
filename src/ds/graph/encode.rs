use std::hash::Hash;
use crate::ds::graph::{Graph, Node, Rooted};

impl<T: Eq + Hash + Copy> Node<T> {
    pub fn encode(&self) -> String {
        let mut encodings = self.neighbours().into_iter()
            .map(|n| n.encode())
            .collect::<Vec<_>>();
        encodings.sort();
        format!("({})", encodings.join(""))
    }
}


impl<T: Eq + Hash + Copy> Graph<T, Rooted> {
    pub fn encode(&self) -> String {
        self.root().encode()
    }
}