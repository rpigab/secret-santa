use rand::seq::SliceRandom;

use crate::graph::digraph::Digraph;
use crate::graph::node::NodeId;

pub fn solve(g: &Digraph) -> Result<Vec<NodeId>, &'static str> {
    // get first node
    let first_node = g.nodes().iter()
        .find(|&n| *n.id() == 0).unwrap()
        .id().clone();

    // find targets of first node
    let solutions = r_find_cycles(g, vec![first_node]);

    log::info!("{} solutions found", solutions.len());
    for cycle in &solutions {
        log::debug!("sol: {cycle:?}");
    }

    if solutions.is_empty() {
        return Err("no solution found");
    }

    let solution = solutions.choose(&mut rand::thread_rng())
        .ok_or("error choosing a random solution")?;
    Ok(solution.clone())
}


/// Find complete cycles,
/// e.g. A->B->C and C->B->A in a graph with 3 nodes with all edges in both directions.
fn r_find_cycles(g: &Digraph, visited: Vec<NodeId>) -> Vec<Vec<NodeId>> {
    let first_node = *visited.first().unwrap();
    let current_node = *visited.last().unwrap();
    let is_last_depth = visited.len() == g.nodes().len();

    if is_last_depth {
        // if last edge loops back to first node, return visited + current as solution
        if g.targets(current_node).iter()
            .find(|&t| *t == first_node).is_some() {
            return vec![visited];
        }
    }

    let mut all_cycles: Vec<Vec<NodeId>> = vec![];

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
