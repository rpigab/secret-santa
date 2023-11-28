use crate::graph::digraph::Digraph;
use crate::graph::node::Node;

pub fn generate_graph(num_nodes: usize) -> Digraph {
    let mut g = Digraph::new();
    let mut curr_idx = 0;

    // add nodes
    for _ in 0..num_nodes {
        g.add_node(Node::new(curr_idx, format!("{curr_idx}")));
        curr_idx += 1;
    }

    // add edges
    g.add_all_edges();
    log::info!("number of edges (complete graph): {}", g.num_edges());

    g
}
