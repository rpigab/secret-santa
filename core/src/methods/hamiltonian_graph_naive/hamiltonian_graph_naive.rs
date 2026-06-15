use anyhow::Result;
use thiserror::Error;

use crate::data::generate_data::generate_graph;
use crate::data::participants_data::ParticipantsData;
use crate::graph::digraph::Digraph;
use crate::methods::hamiltonian_graph_naive::hamiltonian_graph_naive::HamiltonianGraphNaiveError::TooManyParticipants;
use crate::methods::hamiltonian_graph_naive::solve_cycles::solve;
use crate::methods::solve_method::SolveMethod;
use crate::solution::Solution;

const MAX_PARTICIPANTS: u8 = 10;

#[derive(Debug)]
pub(crate) struct HamiltonianGraphNaive {}

#[derive(Debug, Error)]
pub enum HamiltonianGraphNaiveError {
    #[error("the naive algorithm isn't suitable over {MAX_PARTICIPANTS} participants")]
    TooManyParticipants,
}

impl SolveMethod for HamiltonianGraphNaive {
    fn solve(&self, participants_data: ParticipantsData) -> Result<Solution> {
        if participants_data.participants.len() > MAX_PARTICIPANTS as usize {
            return Err(TooManyParticipants.into());
        }

        let graph: Digraph = participants_data.try_into()?;

        Self::solve_graph(graph)
    }

    fn solve_benchmark(&self, num_nodes: usize) -> Result<Solution> {
        if num_nodes > MAX_PARTICIPANTS as usize {
            return Err(TooManyParticipants.into());
        }

        let graph: Digraph = generate_graph(num_nodes);

        Self::solve_graph(graph)
    }
}

impl HamiltonianGraphNaive {
    fn solve_graph(graph: Digraph) -> Result<Solution> {
        let cycle = solve(&graph)?;

        let links = Solution::from_cycle(graph, cycle)?;
        Ok(links)
    }
}
