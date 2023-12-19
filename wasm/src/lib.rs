use std::cmp::max;

use console_log;
use log::Level;
use wasm_bindgen::prelude::*;

use secret_santa_core::solution::Assignment;
use secret_santa_core::solution::Solution;
use secret_santa_core::solve::solve_from_data;
use secret_santa_utils::obfuscate;
use secret_santa_utils::obfuscate::{generate_random_seed, obfuscate_name};

use crate::utils::set_panic_hook;

mod utils;

const AFFECTATION_BASE_URI: &str = "fr/affectation-v2.html";

#[wasm_bindgen]
pub fn init() {
    set_panic_hook();
    console_log::init_with_level(Level::Debug).expect("error init log");
}

#[wasm_bindgen]
pub struct WasmSolution(Solution);

#[wasm_bindgen]
pub fn solve(input_data: String) -> Result<WasmSolution, String> {
    log::debug!("solve");
    let res = solve_from_data(input_data, "HamiltonianGraphNaive".to_string())
        .map_err(|e| format!("{e}"))?;
    Ok(WasmSolution(res))
}

#[wasm_bindgen]
pub fn show_result_html(solution: WasmSolution) -> String {
    log::debug!("show_result_html");
    let WasmSolution(solution) = solution;
    let max_length = solution.assignments().iter()
        .fold(0_usize, |a, Assignment { recipient, .. }| max(a, recipient.len()));

    let seed = generate_random_seed();

    let res = solution.assignments().into_iter()
        .map(|Assignment { recipient, giver }| {
            //TODO obfuscate recipient
            let recipient_obf = obfuscate_name(recipient, seed, max_length);
            log::debug!("giver:{giver}, recipient:{recipient}, recipient_obf:{recipient_obf}, seed:{seed}");
            let uri = format!("{AFFECTATION_BASE_URI}?g={giver}&s={seed}&r={recipient_obf}");
            format!(r#"<li><a href="{uri}" target="_blank">{giver}</a></li>"#)
        }).fold(String::new(), |a, b| a + &b + "\n");
    res
}

#[wasm_bindgen]
pub fn deobfuscate_name(name: String, seed: String) -> String {
    log::debug!("deobfuscate_name({name}, {seed})");
    obfuscate::deobfuscate_name(&*name, &*seed)
}

#[wasm_bindgen]
pub fn hello() -> String {
    log::debug!("hello()");
    "abc".to_string()
}
