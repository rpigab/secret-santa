use std::collections::HashSet;

use anyhow::Result;

use secret_santa_utils::recipient_uri::build_recipient_uri;

use crate::assignment_links::{AssignmentLink, AssignmentLinks};
use crate::cycle::Cycle;
use crate::graph::digraph::Digraph;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Assignment {
    giver: String,
    recipient: String,
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

    pub(crate) fn to_solution_links(self) -> AssignmentLinks {
        let assignments_links = self.assignments.into_iter()
            .map(|Assignment { giver, recipient }|
                (
                    AssignmentLink {
                        giver_name: giver.clone(),
                        recipient_link: build_recipient_uri(giver, recipient),
                    }
                )
            )
            .collect();

        AssignmentLinks { assignments_links }
    }
}
