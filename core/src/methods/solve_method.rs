use std::fmt::Debug;

use anyhow::anyhow;

use crate::data::participants_data::ParticipantsData;
use crate::error::Grinch;
use crate::methods::adjacency_matrix::adjacency_matrix::AdjacencyMatrix;
use crate::methods::hamiltonian_backtrack::hamiltonian_backtrack::HamiltonianBacktrack;
use crate::methods::hamiltonian_graph_naive::hamiltonian_graph_naive::HamiltonianGraphNaive;
use crate::methods::split_quatuors::split_quatuors::SplitQuatuors;
use crate::solution::Solution;

pub(crate) trait SolveMethod: Debug {
    fn solve(&self, participants_data: ParticipantsData) -> anyhow::Result<Solution>;

    fn solve_benchmark(&self, num_nodes: usize) -> anyhow::Result<Solution>;
}

impl TryFrom<String> for Box<dyn SolveMethod> {
    type Error = anyhow::Error;

    fn try_from(method_name: String) -> Result<Self, Self::Error> {
        let method: Box<dyn SolveMethod> = match method_name.as_str() {
            "AdjacencyMatrix" => Box::new(AdjacencyMatrix {}),
            "HamiltonianGraphNaive" => Box::new(HamiltonianGraphNaive {}),
            "HamiltonianBacktrack" => Box::new(HamiltonianBacktrack {}),
            "SplitQuatuors" => Box::new(SplitQuatuors {}),
            _ => Err(anyhow!(Grinch::InvalidMethodName{method_name}))?,
        };
        Ok(method)
    }
}
