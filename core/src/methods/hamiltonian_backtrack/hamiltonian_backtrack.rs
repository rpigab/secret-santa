use anyhow::Result;

use crate::data::generate_data::generate_graph;
use crate::data::participants_data::ParticipantsData;
use crate::graph::digraph::Digraph;
use crate::methods::hamiltonian_backtrack::find_cycle::solve;
use crate::methods::solve_method::SolveMethod;
use crate::solution::Solution;

/// Scalable single-cycle solver: finds one random Hamiltonian cycle through all
/// participants via randomized backtracking, without the participant ceiling of
/// the naive enumerator or the small-group fragmentation of split-quatuors.
#[derive(Debug)]
pub(crate) struct HamiltonianBacktrack {}

impl SolveMethod for HamiltonianBacktrack {
    fn solve(&self, participants_data: ParticipantsData) -> Result<Solution> {
        let graph: Digraph = participants_data.try_into()?;
        Self::solve_graph(graph)
    }

    fn solve_benchmark(&self, num_nodes: usize) -> Result<Solution> {
        let graph = generate_graph(num_nodes);
        Self::solve_graph(graph)
    }
}

impl HamiltonianBacktrack {
    fn solve_graph(graph: Digraph) -> Result<Solution> {
        let cycle = solve(&graph)?;
        Solution::from_cycle(graph, cycle)
    }
}
