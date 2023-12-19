use anyhow::{anyhow, ensure, Result};
use rand::seq::SliceRandom;

use secret_santa_utils::bench;

use crate::cycle::Cycle;
use crate::error::Grinch;
use crate::graph::digraph::Digraph;
use crate::graph::node::NodeId;

pub fn solve(g: &Digraph) -> Result<Cycle> {
    ensure!(g.nodes().len() > 2, Grinch::CannotSolveGraph{ reason: "contains less than 3 nodes".to_string() });

    // get first node
    let first_node = g.nodes().iter()
        .find(|&n| *n.id() == 0)
        .unwrap()
        .id().clone();

    // find targets of first node
    let solutions = r_find_cycles(g, vec![first_node]);

    log::info!("{} solutions found", solutions.len());
    for cycle in &solutions {
        log::trace!("sol: {cycle:?}");
    }

    ensure!(!solutions.is_empty(), Grinch::NoSolutionFound);

    let solution = solutions.choose(&mut rand::thread_rng())
        .ok_or(anyhow!(Grinch::RandomPickFailed))?;

    Ok(solution.clone())
}


/// Find complete cycles,
/// e.g. A->B->C and C->B->A in a graph with 3 nodes with all edges in both directions.
fn r_find_cycles(g: &Digraph, visited: Vec<NodeId>) -> Vec<Cycle> {
    let first_node = *visited.first().unwrap();
    let current_node = *visited.last().unwrap();
    let is_last_depth = visited.len() == g.nodes().len();

    if is_last_depth {
        // if last edge loops back to first node, return visited + current as solution
        if g.targets(current_node).iter()
            .find(|&t| *t == first_node).is_some() {
            bench::alloc::check_current_alloc();
            return vec![Cycle::from_vec(visited)];
        }
    }

    let mut all_cycles: Vec<Cycle> = vec![];

    // iterate on neighbors
    for next in g.targets(current_node) {
        if !visited.contains(&next) {
            let mut next_visited = visited.clone();
            next_visited.push(next);
            let cycles = r_find_cycles(g, next_visited);
            all_cycles.extend(cycles);
        }
    }

    all_cycles
}
