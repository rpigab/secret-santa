use prettytable::{row, table};

use secret_santa_core::assignment_links::AssignmentLinks;
use secret_santa_core::assignment_links::AssignmentLink;

pub fn display_links_table(links: AssignmentLinks, affectation_base_uri: String) {
    let mut table = table!([rFg->"Giver name", Fg->"Link"]);


    for AssignmentLink { giver_name, recipient_link } in links.assignments_links() {
        table.add_row(row![r->giver_name, format!("{affectation_base_uri}{recipient_link}")]);
    }

    log::info!("Affectations as links to send to each gift giver:\n{table}");
}
