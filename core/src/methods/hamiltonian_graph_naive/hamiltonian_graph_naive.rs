use anyhow::Result;

use crate::data::generate_data::generate_graph;
use crate::data::participants_data::ParticipantsData;
use crate::graph::digraph::Digraph;
use crate::methods::hamiltonian_graph_naive::solve_cycles::solve;
use crate::methods::solve_method::SolveMethod;
use crate::solution::Solution;

#[derive(Debug)]
pub(crate) struct HamiltonianGraphNaive {}

impl SolveMethod for HamiltonianGraphNaive {
    fn solve(&self, participants_data: ParticipantsData) -> Result<Solution> {
        let graph: Digraph = participants_data.try_into()
            .expect("error");
        let cycle = solve(&graph)?;

        let links = Solution::from_cycle(graph, cycle)?;
        Ok(links)
    }

    fn solve_benchmark(&self, num_nodes: usize) -> Result<Solution> {
        let graph: Digraph = generate_graph(num_nodes);
        let cycle = solve(&graph)?;

        let links = Solution::from_cycle(graph, cycle)?;
        Ok(links)
    }
}
