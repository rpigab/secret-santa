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

const DEFAULT_METHOD: &str = "HamiltonianBacktrack";

#[wasm_bindgen]
pub fn solve(input_data: String, method_name: String) -> Result<WasmSolution, String> {
    log::debug!("solve (method: {method_name})");
    let method_name = if method_name.is_empty() {
        DEFAULT_METHOD.to_string()
    } else {
        method_name
    };
    let res = solve_from_data(input_data, method_name)
        .map_err(|e| format!("{e}"))?;
    Ok(WasmSolution(res))
}

// NOTE: takes `&WasmSolution` (by reference) instead of by value so the JS
// frontend can read both the obfuscated links AND the debug assignments from
// the *same* solve() — the solver picks a cycle at random, so re-solving would
// give arrows that don't match the links.
#[wasm_bindgen]
pub fn show_result_html(solution: &WasmSolution) -> String {
    log::debug!("show_result_html");
    let solution = &solution.0;
    let max_length = solution.assignments().iter()
        .fold(0_usize, |a, Assignment { recipient, .. }| max(a, recipient.len()));

    let seed = generate_random_seed();

    let res = solution.assignments().into_iter()
        .map(|Assignment { recipient, giver }| {
            let recipient_obf = obfuscate_name(recipient, seed, max_length);
            log::debug!("giver:{giver}, recipient:{recipient}, recipient_obf:{recipient_obf}, seed:{seed}");
            let uri = format!("{AFFECTATION_BASE_URI}?g={giver}&s={seed}&r={recipient_obf}");
            format!(r#"<li><a href="{uri}" target="_blank">{giver}</a></li>"#)
        }).fold(String::new(), |a, b| a + &b + "\n");
    res
}

/// Dev-only: return the *plaintext* assignments as a JSON array
/// `[{"giver":"…","recipient":"…"}, …]`, so the frontend "debug / reveal"
/// mode can draw the real who-gifts-whom arrows. This intentionally bypasses
/// the name obfuscation, so it must only be used behind the UI's debug flag —
/// never in the links that get sent to participants.
#[wasm_bindgen]
pub fn show_result_debug(solution: &WasmSolution) -> String {
    log::debug!("show_result_debug");
    let solution = &solution.0;
    let items: Vec<String> = solution.assignments().iter()
        .map(|Assignment { giver, recipient }| {
            format!(
                "{{\"giver\":{},\"recipient\":{}}}",
                json_string(giver),
                json_string(recipient)
            )
        })
        .collect();
    format!("[{}]", items.join(","))
}

/// Minimal JSON string encoder (avoids pulling in serde_json for one helper).
fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[wasm_bindgen]
pub fn deobfuscate_name(name: String, seed: String) -> String {
    log::debug!("deobfuscate_name({name}, {seed})");
    obfuscate::deobfuscate_name(&*name, &*seed)
}
