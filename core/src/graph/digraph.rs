use std::collections::{HashMap, HashSet};

use crate::graph::node::{Node, NodeId};

#[derive(Debug)]
pub struct Digraph {
    /// Nodes of the graph
    nodes: HashSet<Node>,
    /// Edges of the directed graph.
    ///
    /// Map with source nodes as keys,
    /// each target in the HashSet is an edge from the source key
    edges: HashMap<NodeId, HashSet<NodeId>>,
}

impl Digraph {
    pub fn new() -> Self {
        Digraph {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.edges.insert(*node.id(), HashSet::new());
        self.nodes.insert(node);
    }

    pub fn add_all_edges(&mut self) {
        let sources: Vec<NodeId> = self.nodes().iter()
            .map(|n| n.id().clone())
            .collect();
        let targets: Vec<NodeId> = self.nodes().iter()
            .map(|n| n.id().clone())
            .collect();
        sources.iter().for_each(|source| {
            targets.iter().for_each(|target| if source != target {
                self.add_edge(*source, *target).unwrap();
            });
        });
    }


    pub fn add_edge(&mut self, u: NodeId, v: NodeId) -> Result<(), &'static str> {
        // Only add edges if nodes u and v exist
        // and if edge doesn't already exist
        if self.nodes.iter().find(|&n| *n.id() == u).is_some()
            && self.nodes.iter().find(|&n| *n.id() == v).is_some() {
            // Add edge
            let edges_by_source = self.edges.get_mut(&u);
            match edges_by_source {
                None => {
                    return Err("Edge should contain HashSet from add_node()");
                }
                Some(targets) => {
                    // Add to set
                    targets.insert(v);
                }
            }
            Ok(())
        } else {
            Err("Cannot add edge: both nodes must exist")
        }
    }

    pub(crate) fn remove_edges_both_dirs(&mut self, a: String, b: String)
                                         -> Result<(), &'static str> {
        self.remove_edge(&a, &b)?;
        self.remove_edge(&b, &a)?;
        Ok(())
    }

    pub(crate) fn remove_edge(&mut self, a: &String, b: &String)
                              -> Result<(), &'static str> {
        let a = self.node_name_to_id(a)?;
        let b = self.node_name_to_id(b)?;

        self.remove_edge_ids(a, b)
    }

    pub(crate) fn remove_edge_ids(&mut self, a: NodeId, b: NodeId)
                                  -> Result<(), &'static str> {
        let foo = self.edges.get_mut(&a).unwrap();
        let res = foo.remove(&b);
        if !res {
            return Err("tried to remove edge that did not exist");
        }
        Ok(())
    }

    pub fn targets(&self, source_node: NodeId) -> HashSet<NodeId> {
        self.edges.get(&source_node)
            .unwrap().clone()
    }

    pub fn node_name_to_id<'a>(&self, name: &String) -> Result<NodeId, &'static str> {
        Ok(*self.nodes.iter().find(|&n| *n.name() == *name)
            .ok_or("cannot find node")?.id())
    }

    pub(crate) fn node_id_to_name(&self, id: NodeId) -> Result<String, &'static str> {
        Ok(self.nodes.iter().find(|&n| *n.id() == id)
            .ok_or("cannot find node")?.name().clone())
    }

    pub fn nodes(&self) -> &HashSet<Node> {
        &self.nodes
    }

    pub fn num_edges(&self) -> usize {
        self.edges.iter()
            .map(|e| e.1)
            .map(HashSet::len)
            .sum()
    }
}
