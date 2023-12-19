use std::cmp::max;

use prettytable::{row, table};

use secret_santa_core::solution::{Assignment, Solution};
use secret_santa_utils::obfuscate::generate_random_seed;
use secret_santa_utils::recipient_uri::build_recipient_uri;

pub fn display_links_table(solution: Solution, affectation_base_uri: String) {
    let mut table = table!([rFg->"Giver name", Fg->"Link"]);
    let assignments = solution.assignments();

    let seed = generate_random_seed();
    let max_length = assignments.iter()
        .fold(0, |a, Assignment { recipient, .. }| max(a, recipient.len()));

    assignments.iter()
        .for_each(|Assignment { giver, recipient }| {
            let uri = build_recipient_uri(&*affectation_base_uri, giver, recipient, seed, max_length);
            table.add_row(row![r->giver, uri]);
        });

    log::info!("Affectations as links to send to each gift giver:\n{table}");
}
