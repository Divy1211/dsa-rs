use std::iter::from_fn;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

use crate::ds::graph::{Graph};

impl<T: Eq + Hash + Copy> Graph<T> {
    pub fn root(&self) -> Graph<T> {
        let centre = self.centre()[0].clone();
        
        let mut visited = HashSet::with_capacity(self.nodes.len());
        let mut deq = VecDeque::from([centre]);

        let edges = from_fn(|| deq.pop_front().map(|node| {
            visited.insert(node.id);
            
            node.neighbours().into_iter()
                .filter(|neighbour| !visited.contains(&neighbour.id))
                .map(|neighbour| {
                    deq.push_back(neighbour.clone());
                    (node.id, neighbour.id)
                }).collect::<Vec<_>>()
            
        })).flatten()
        .collect();

        Graph::from_edge_list(edges, false)
    }
}