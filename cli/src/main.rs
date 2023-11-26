use std::path::PathBuf;

use clap::Parser;
use env_logger::Env;

use secret_santa_core::solve_file;

/// Load a yaml data file into a graph of participants in a Secret Santa activity,
/// then select at random one of the solutions to assign givers and recipients
/// while respecting a set of constraints,
/// finally, return a list of URLs the the called should send the right link to each giver.
#[derive(Parser)]
struct CliOpts {
    /// The path to the file to read
    input_file: PathBuf,
    /// The base URI to generate affectation links,
    /// e.g. "http://localhost:8000/fr/affectation.html" (which is the default, if not provided)
    affectation_base_uri: Option<String>,
}

const DEFAULT_AFFECTATION_BASE_URI: &str = "http://localhost:8000/fr/affectation.html";

fn main() {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let cli_opts = CliOpts::parse();

    let affectation_base_uri = cli_opts.affectation_base_uri
        .unwrap_or_else(|| {
            log::warn!("setting affectation_base_uri to default localhost value");
            DEFAULT_AFFECTATION_BASE_URI.to_string()
        });

    match solve_file(cli_opts.input_file, affectation_base_uri) {
        Ok(_) => {}
        Err(e) => eprint!("error: {e}")
    }
}
