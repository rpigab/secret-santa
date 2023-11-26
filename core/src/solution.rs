use std::collections::{HashMap, HashSet};

use base64::Engine;
use base64::engine::general_purpose;
use url::Url;

use crate::graph::digraph::Digraph;
use crate::graph::node::NodeId;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Assignment {
    giver: String,
    recipient: String,
}

#[derive(Debug)]
pub struct Solution {
    assignments: HashSet<Assignment>,
    affectation_base_uri: String,
}

#[derive(Debug)]
pub struct AssignmentLink {
    giver: String,
    link: String,
}

impl Solution {
    pub fn get_solution(g: Digraph, cycle: Vec<NodeId>, affectation_base_uri: String)
                        -> Result<Self, &'static str> {
        log::debug!("solution: {cycle:?}");

        let assignments = (0..cycle.len()).into_iter()
            .map(|i| {
                let giver = cycle[i];
                let giver = g.node_id_to_name(giver).unwrap();
                let recipient = cycle[(i + 1) % cycle.len()];
                let recipient = g.node_id_to_name(recipient).unwrap();

                Assignment { giver, recipient }
            }).collect::<HashSet<Assignment>>();

        Ok(Solution {
            assignments,
            affectation_base_uri,
        })
    }

    pub fn display_links(&self) -> Result<HashMap<String, String>, &'static str> {
        let links: Result<HashMap<String, String>, &'static str> = self.assignments.iter()
            .map(|a| build_uri(self.affectation_base_uri.as_str(), a.giver.as_str(), a.recipient.as_str()))
            .map(|r| r.map(|a| (a.giver, a.link)))
            .collect();

        links
    }
}


fn build_uri(base_path: &str, giver: &str, recipient: &str) -> Result<AssignmentLink, &'static str> {
    let mut url = Url::parse(base_path)
        .map_err(|_| "parse error in url")?;
    url.query_pairs_mut().append_pair("giver", giver);

    let recipient_b64 = general_purpose::STANDARD.encode(recipient);
    url.query_pairs_mut().append_pair("recipient", &*recipient_b64);

    Ok(AssignmentLink {
        giver: giver.to_string(),
        link: url.to_string(),
    })
}
