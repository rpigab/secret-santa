use std::collections::HashMap;
use crate::data::participants_data::ParticipantsData;
use crate::graph::digraph::Digraph;
use crate::graph::node::{Node, NodeId};

impl TryFrom<ParticipantsData> for Digraph {
    type Error = &'static str;

    fn try_from(value: ParticipantsData) -> Result<Self, Self::Error> {
        let mut g = Digraph::new();
        let mut curr_idx = 0;

        // add nodes
        for p in value.participants.into_iter() {
            g.add_node(Node::new(curr_idx, p));
            curr_idx += 1;
        }

        // add edges
        let sources: Vec<NodeId> = g.nodes().iter()
            .map(|n| n.id().clone())
            .collect();
        let targets: Vec<NodeId> = g.nodes().iter()
            .map(|n| n.id().clone())
            .collect();
        sources.iter().for_each(|source| {
            targets.iter().for_each(|target| if source != target {
                g.add_edge(*source, *target).unwrap();
            });
        });
        log::info!("number of edges to begin (complete graph): {}", g.num_edges());

        // remove edges for couples
        let couples = match value.couples {
            None => {
                log::warn!("no `couples` defined in input file");
                vec![]
            }
            Some(couples) => {
                couples
            }
        };
        for (a, b) in couples {
            // remove edges a -> b and b -> a
            g.remove_edges_both_dirs(a, b)?;
        }
        log::info!("number of edges after couples pruning: {}", g.num_edges());

        // remove edges for ppl who already gave to the same person
        let already_gifted_before = match value.already_gifted_before {
            None => {
                log::warn!("no `already_gifted_before` defined in input file");
                HashMap::new()
            }
            Some(already_gifted_before) => {
                already_gifted_before
            }
        };
        for (giver, prev_recipients) in already_gifted_before {
            let giver = g.node_name_to_id(&giver)?;
            for recipient in prev_recipients {
                let recipient = g.node_name_to_id(&recipient)?;
                // remove edge `giver` -> `recipient`
                g.remove_edge_ids(giver, recipient)?;
            }
        }
        log::info!("number of edges after `already_gifted_before` pruning: {}", g.num_edges());
        Ok(g)
    }
}
