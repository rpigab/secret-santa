use console_log;
use log::Level;
use wasm_bindgen::prelude::*;

use secret_santa_core::assignment_links::{AssignmentLink, AssignmentLinks};
use secret_santa_core::solve::solve_from_data;

use crate::utils::set_panic_hook;

mod utils;

const AFFECTATION_BASE_URI: &str = "fr/affectation.html";

#[wasm_bindgen]
pub fn init() {
    set_panic_hook();
    console_log::init_with_level(Level::Debug).expect("error init log");
}

#[wasm_bindgen]
pub struct WasmAssignmentLinks(AssignmentLinks);

#[wasm_bindgen]
pub fn solve(input_data: String) -> Result<WasmAssignmentLinks, String> {
    log::debug!("solve");
    let res = solve_from_data(input_data, "HamiltonianGraphNaive".to_string())
        .map_err(|e| format!("{e}"))?;
    Ok(WasmAssignmentLinks(res))
}

#[wasm_bindgen]
pub fn show_result_html(assignment_links: WasmAssignmentLinks) -> String {
    log::debug!("show_result_html");
    let WasmAssignmentLinks(links) = assignment_links;
    let res = links.assignments_links().into_iter()
        .map(|AssignmentLink { giver_name, recipient_link }| {
            format!(r#"<li><a href="{AFFECTATION_BASE_URI}{recipient_link}" target="_blank">{giver_name}</a></li>"#)
        }).fold(String::new(), |a, b| a + &b + "\n");
    res
}
