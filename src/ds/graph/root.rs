use std::iter::from_fn;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use std::rc::Rc;
use crate::ds::graph::{Graph, Node, Rooted, Unrooted};

impl<T: Eq + Hash + Copy> Graph<T, Unrooted> {
    pub fn root_from(&self, start: &Rc<Node<T>>) -> Graph<T, Rooted> {
        let mut visited = HashSet::with_capacity(self.nodes.len());
        let mut deq = VecDeque::from([start.clone()]);

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
            .root_at(start.id)
    }
    pub fn root(&self) -> Graph<T, Rooted> {
        let centre = self.centre()[0].clone();
        self.root_from(&centre)
    }
}