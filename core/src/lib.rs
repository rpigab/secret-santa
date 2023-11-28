use std::collections::HashMap;

use crate::data::generate_data::generate_graph;
use crate::data::participants_data::ParticipantsData;
use crate::graph::digraph::Digraph;
use crate::solution::Solution;
use crate::solve::solve;

mod solution;
mod solve;
mod graph;
mod data;

pub fn solve_from_data<D: TryInto<ParticipantsData>>(input_data: D, affectation_base_uri: String)
                                                  -> Result<HashMap<String, String>, &'static str>
    where <D as TryInto<ParticipantsData>>::Error: std::fmt::Debug {
    log::debug!("solve_from_data()");
    let data: ParticipantsData = input_data.try_into().expect("error");
    log::debug!("ParticipantData loaded");
    let graph: Digraph = data.try_into()?;
    log::debug!("Digraph loaded");
    let cycle = solve(&graph)?;

    let sol = Solution::get_solution(graph, cycle, affectation_base_uri)?;
    log::debug!("solution: {sol:#?}");
    let links = sol.display_links()?;
    Ok(links)
}

pub fn benchmark_solve(num_nodes: usize, affectation_base_uri: String)
                       -> Result<HashMap<String, String>, &'static str> {
    let graph: Digraph = generate_graph(num_nodes);
    let cycle = solve(&graph)?;

    let sol = Solution::get_solution(graph, cycle, affectation_base_uri)?;
    log::debug!("solution: {sol:#?}");
    let links = sol.display_links()?;
    Ok(links)
}
