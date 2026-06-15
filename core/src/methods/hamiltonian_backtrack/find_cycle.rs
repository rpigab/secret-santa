use anyhow::{ensure, Result};
use rand::seq::SliceRandom;
use rand::Rng;

use crate::cycle::Cycle;
use crate::error::Grinch;
use crate::graph::digraph::Digraph;
use crate::graph::node::NodeId;

/// Find a single Hamiltonian cycle using randomized depth-first backtracking.
///
/// Unlike the naive method, this does not enumerate every cycle: it walks the
/// graph trying random unvisited neighbours and returns the first complete cycle
/// it can close back to the start. On the near-complete graphs produced by a
/// secret santa (a complete digraph minus a handful of forbidden edges) almost
/// any choice works, so it runs in roughly linear time and scales to hundreds of
/// participants, while still picking a different solution on each run. It also
/// produces one big cycle rather than the small, easily-guessable groups of the
/// split-quatuors method.
pub fn solve(g: &Digraph) -> Result<Cycle> {
    let n = g.nodes().len();
    ensure!(n > 2, Grinch::CannotSolveGraph { reason: "contains less than 3 nodes".to_string() });

    // node ids are contiguous 0..n (see data_to_digraph / generate_graph)
    let start: NodeId = 0;
    let mut visited = vec![false; n];
    let mut path: Vec<NodeId> = Vec::with_capacity(n);

    visited[start as usize] = true;
    path.push(start);

    let mut rng = rand::thread_rng();
    ensure!(
        find(g, start, &mut visited, &mut path, n, &mut rng),
        Grinch::NoSolutionFound
    );

    log::info!("found a hamiltonian cycle over {n} nodes");
    Ok(Cycle::from_vec(path))
}

/// Depth-first search that extends `path` until it covers all `n` nodes and can
/// close back to `start`. Returns `true` (with `path` holding the solution) as
/// soon as one valid cycle is found.
fn find(
    g: &Digraph,
    start: NodeId,
    visited: &mut [bool],
    path: &mut Vec<NodeId>,
    n: usize,
    rng: &mut impl Rng,
) -> bool {
    let current = *path.last().unwrap();

    if path.len() == n {
        // close the cycle: the last node must have an edge back to the start
        return g.targets(current).contains(&start);
    }

    let mut candidates: Vec<NodeId> = g.targets(current).into_iter()
        .filter(|target| !visited[*target as usize])
        .collect();
    candidates.shuffle(rng);

    for next in candidates {
        visited[next as usize] = true;
        path.push(next);

        if find(g, start, visited, path, n, rng) {
            return true;
        }

        path.pop();
        visited[next as usize] = false;
    }

    false
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::data::generate_data::generate_graph;

    #[test]
    fn finds_valid_cycle_on_large_complete_graph() {
        let n = 100;
        let g = generate_graph(n);

        let cycle = solve(&g).unwrap();

        assert_eq!(cycle.len(), n);

        // every node visited exactly once, and every consecutive pair is an edge
        let mut seen = HashSet::new();
        for i in 0..cycle.len() {
            assert!(seen.insert(cycle[i]), "node visited twice");
            let from = cycle[i];
            let to = cycle[(i + 1) % cycle.len()];
            assert!(g.targets(from).contains(&to), "missing edge in cycle");
        }
    }

    #[test]
    fn rejects_graphs_too_small() {
        let g = generate_graph(2);
        assert!(solve(&g).is_err());
    }
}
