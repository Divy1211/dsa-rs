use std::hash::Hash;
use crate::ds::graph::{Graph, Unrooted};

impl<U: Eq + Hash + Copy> Graph<U, Unrooted> {
    pub fn is_isomorphic_to<V: Eq + Hash + Copy>(&self, other: &Graph<V, Unrooted>) -> bool {
        let rooted1 = self.root();
        let encoding1 = rooted1.encode();
        let centres2 = other.centre();

        for centre in centres2 {
            let rooted2 = other.root_from(&centre);
            if rooted2.encode() == encoding1 {
                return true;
            }
        }
        false
    }
}