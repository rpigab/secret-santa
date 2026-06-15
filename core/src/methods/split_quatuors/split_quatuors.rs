use std::collections::HashSet;

use anyhow::Result;

use crate::data::participants_data::ParticipantsData;
use crate::graph::digraph::Digraph;
use crate::methods::hamiltonian_graph_naive::solve_cycles::solve;
use crate::methods::solve_method::SolveMethod;
use crate::solution::Solution;
use crate::split_participants::split::split_participants;

#[derive(Debug)]
pub(crate) struct SplitQuatuors {}

impl SolveMethod for SplitQuatuors {
    fn solve(&self, participants_data: ParticipantsData) -> Result<Solution> {
        // split participants into independent subgroups (quatuors + complement)
        let groups = split_participants(participants_data);
        Self::solve_groups(groups)
    }

    fn solve_benchmark(&self, num_nodes: usize) -> Result<Solution> {
        let participants_data = ParticipantsData {
            participants: (0..num_nodes).map(|i| i.to_string()).collect(),
            already_gifted_before: None,
            couples: None,
        };

        self.solve(participants_data)
    }
}

impl SplitQuatuors {
    /// Solve each subgroup independently as a Hamiltonian cycle, then merge the
    /// per-group assignments. Disjoint cycles covering every participant still
    /// form a valid secret-santa derangement.
    fn solve_groups(groups: Vec<ParticipantsData>) -> Result<Solution> {
        let mut assignments = HashSet::new();

        for group in groups {
            let graph: Digraph = group.try_into()?;
            let cycle = solve(&graph)?;
            let group_solution = Solution::from_cycle(graph, cycle)?;
            assignments.extend(group_solution.into_assignments());
        }

        Ok(Solution::from_assignments(assignments))
    }
}
