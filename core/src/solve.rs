use anyhow::{anyhow, Result};

use crate::data::participants_data::ParticipantsData;
use crate::error::Grinch;
use crate::methods::hamiltonian_graph_naive::hamiltonian_graph_naive::HamiltonianGraphNaive;
use crate::methods::solve_method::SolveMethod;
use crate::solution::Solution;

pub fn solve_from_data<D: TryInto<ParticipantsData>>(input_data: D, method_name: String)
                                                     -> Result<Solution> {
    log::debug!("solve_from_data()");
    let data: ParticipantsData = input_data.try_into()
        .map_err(|_| anyhow!(Grinch::CouldNotBuildInputData))?;
    let method: Box<dyn SolveMethod> = method_name.try_into()?;
    log::debug!("method: {method:?}");
    let solution = method.solve(data)?;
    Ok(solution)
}

pub fn benchmark_solve(num_nodes: usize) -> Result<Solution> {
    let method = Box::new(HamiltonianGraphNaive {});
    let solution = method.solve_benchmark(num_nodes)?;
    Ok(solution)
}
