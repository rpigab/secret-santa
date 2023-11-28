use std::collections::HashMap;

use prettytable::{row, table};

pub fn display_links_table(links: HashMap<String, String>) {
    let mut table = table!([rFg->"Giver name", Fg->"Link"]);

    for (giver, link) in links {
        table.add_row(row![r->giver, link]);
    }

    log::info!("Affectations as links to send to each gift giver:\n{table}");
}
