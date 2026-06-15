use std::collections::HashSet;

use anyhow::Result;

use crate::cycle::Cycle;
use crate::graph::digraph::Digraph;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Assignment {
    pub giver: String,
    pub recipient: String,
}

#[derive(Debug)]
pub struct Solution {
    assignments: HashSet<Assignment>,
}

impl Solution {
    pub(crate) fn from_cycle(g: Digraph, cycle: Cycle) -> Result<Self> {
        log::debug!("solution: {cycle:?}");

        let assignments = (0..cycle.len()).into_iter()
            .map(|i| {
                let giver = cycle[i];
                let giver = g.node_id_to_name(giver).unwrap();
                let recipient = cycle[(i + 1) % cycle.len()];
                let recipient = g.node_id_to_name(recipient).unwrap();

                Assignment { giver, recipient }
            }).collect::<HashSet<Assignment>>();

        Ok(Solution {
            assignments,
        })
    }

    /// Build a solution from already-computed assignments,
    /// e.g. when merging the cycles of several independently-solved subgraphs.
    pub(crate) fn from_assignments(assignments: HashSet<Assignment>) -> Self {
        Solution { assignments }
    }

    pub fn assignments(&self) -> &HashSet<Assignment> {
        &self.assignments
    }

    pub(crate) fn into_assignments(self) -> HashSet<Assignment> {
        self.assignments
    }
}
