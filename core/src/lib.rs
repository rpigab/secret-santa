use std::collections::HashMap;
use std::path::PathBuf;

use crate::data::participants_data::ParticipantsData;
use crate::graph::digraph::Digraph;
use crate::solution::Solution;
use crate::solve::solve;

mod solution;
mod solve;
mod graph;
mod data;

pub fn solve_file(input_file: PathBuf, affectation_base_uri: String)
                  -> Result<HashMap<String, String>, &'static str> {
    let data = ParticipantsData::new(input_file);
    let graph: Digraph = data.try_into()?;
    let cycle = solve(&graph)?;

    let sol = Solution::get_solution(graph, cycle, affectation_base_uri)?;
    log::debug!("solution: {sol:#?}");
    let links = sol.display_links()?;
    Ok(links)
}
