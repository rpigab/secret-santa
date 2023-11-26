use std::collections::HashSet;

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

    pub fn get_links(&self) -> Result<Vec<String>, &'static str> {
        let links: Result<Vec<String>, &'static str> = self.assignments.iter()
            .map(|a| build_uri(self.affectation_base_uri.as_str(), a.giver.as_str(), a.recipient.as_str()))
            .map(|r| r.map(|u| Url::to_string(&u)))
            .map(|r| r.map_err(|_| "parse error in url"))
            .collect();

        links
    }
}


fn build_uri(base_path: &str, giver: &str, recipient: &str) -> Result<Url, url::ParseError> {
    let mut url = Url::parse(base_path)?;
    url.query_pairs_mut().append_pair("giver", giver);


    let recipient_b64 = general_purpose::STANDARD.encode(recipient);
    url.query_pairs_mut().append_pair("recipient", &*recipient_b64);

    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_links_ok() {
        let base_path = "http://localhost:8000/";

        match build_uri(base_path, "Machin", "Truc") {
            Ok(uri) => {
                println!("Built URI: {}", uri);
            }
            Err(err) => {
                eprintln!("Error building URI: {}", err);
            }
        }
    }
}
