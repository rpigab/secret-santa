use anyhow::Result;

use crate::data::participants_data::ParticipantsData;
use crate::methods::solve_method::SolveMethod;
use crate::solution::Solution;

#[derive(Debug)]
pub(crate) struct SplitQuatuors {}

impl SolveMethod for SplitQuatuors {
    fn solve(&self, participants_data: ParticipantsData) -> Result<Solution> {
        unimplemented!()
    }

    fn solve_benchmark(&self, num_nodes: usize) -> Result<Solution> {
        unimplemented!()
    }
}
