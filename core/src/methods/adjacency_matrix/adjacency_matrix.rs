use crate::data::participants_data::ParticipantsData;
use crate::methods::solve_method::SolveMethod;
use crate::solution::Solution;
use anyhow::Result;
#[derive(Debug)]
pub(crate) struct AdjacencyMatrix {}

impl SolveMethod for AdjacencyMatrix {
    fn solve(&self, participants_data: ParticipantsData) -> Result<Solution> {
        unimplemented!()
    }

    fn solve_benchmark(&self, num_nodes: usize) -> Result<Solution> {
        unimplemented!()
    }
}
