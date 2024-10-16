use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use crate::ds::graph::{Graph, Node};


impl<T: Eq + Hash + Copy> Graph<T> {
    pub fn in_degrees(&self) -> HashMap<T, usize> {
        let mut in_degrees = HashMap::with_capacity(self.nodes.len());
        
        for (_id, node) in self.nodes.iter() {
            for node2 in node.neighbours() {
                *in_degrees.entry(node2.id).or_insert(0) += 1;
            }
        }

        in_degrees
    }
    
    pub fn centre(&self) -> Vec<Rc<Node<T>>> {
        let mut in_degrees = self.in_degrees();

        while in_degrees.len() > 2 {
            let leaves = in_degrees.iter()
                .filter(|(_, &in_degree)| in_degree == 1)
                .map(|(id, _)| *id)
                .collect::<Vec<_>>();

            for leaf in leaves {
                let neighbours = self.nodes.get(&leaf)
                    .expect("infallible")
                    .neighbours();
                
                for node in neighbours {
                    *in_degrees.get_mut(&node.id).expect("infallible") -= 1;
                }
                in_degrees.remove(&leaf);
            }
        }
        
        in_degrees.iter()
            .map(|(id, _)| {
                self.nodes.get(id).expect("infallible").clone()
            }).collect()
    }
}