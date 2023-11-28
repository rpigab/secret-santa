use console_log;
use log::Level;
use serde_json;
use wasm_bindgen::prelude::*;
use web_sys::window;

use secret_santa_core::solve_from_data;

use crate::utils::set_panic_hook;

mod utils;

#[wasm_bindgen]
pub fn init() {
    set_panic_hook();
    console_log::init_with_level(Level::Debug).expect("error init log");
}

#[wasm_bindgen]
pub fn solve(input_data: String) -> String {
    log::debug!("solve");
    let res = solve_from_data(input_data, get_affectation_base_uri())
        .expect("error solving from data");
    serde_json::to_string(&res)
        .expect("error serializing hashmap result to string")
}

fn get_affectation_base_uri() -> String {
    let s: String = window().unwrap().location().to_string().into();
    log::debug!("location: {s}");
    let res = if let Some(stripped) = s.strip_suffix("/index.html") {
        stripped.to_string()
    } else if let Some(stripped) = s.strip_suffix("/") {
        stripped.to_string()
    } else {
        s.to_string()
    };

    format!("{res}/fr/affectation.html")
}
