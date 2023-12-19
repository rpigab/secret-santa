use std::ops::Index;

use crate::graph::node::NodeId;

#[derive(Clone, Debug)]
pub(crate) struct Cycle(Vec<NodeId>);

impl Cycle {
    pub(crate) fn from_vec(vec: Vec<NodeId>) -> Self {
        Self(vec)
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for Cycle {
    type Output = NodeId;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}
