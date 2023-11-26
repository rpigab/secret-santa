use crate::data::participants_data::ParticipantsData;
use crate::graph::digraph::Digraph;
use crate::solution::Solution;
use crate::solve::solve;

mod solution;
mod solve;
mod graph;
mod data;

pub fn run() -> Result<(), &'static str> {
    let data = ParticipantsData::new("./data/input_prod.yml".to_string());
    let graph: Digraph = data.try_into()?;
    let cycle = solve(&graph)?;
    let sol = Solution::get_solution(graph, cycle)?;
    log::debug!("solution: {sol:#?}");
    let links = sol.get_links()?;
    log::info!("Affectations as links to send to each gift giver:\n{}", links.join("\n"));
    Ok(())
}
