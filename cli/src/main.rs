use std::path::PathBuf;

use clap::Parser;
use env_logger::Env;

use secret_santa_cli::display_links_table;
use secret_santa_core::solve_from_data;
use secret_santa_utils::bench::alloc::check_final_alloc;
use secret_santa_utils::chrono::Chrono;

/// Load a yaml data file into a graph of participants in a Secret Santa activity,
/// then select at random one of the solutions to assign givers and recipients
/// while respecting a set of constraints,
/// finally, return a list of URLs the the called should send the right link to each giver.
#[derive(Parser)]
struct CliOpts {
    /// The path to the file to read
    input_file: PathBuf,
    /// The base URI to generate affectation links,
    /// e.g. "http://localhost:8080/fr/affectation.html" (which is the default, if not provided)
    affectation_base_uri: Option<String>,
}

const DEFAULT_AFFECTATION_BASE_URI: &str = "http://localhost:8080/fr/affectation.html";

fn main() {
    let chrono = Chrono::new();
    run();
    chrono.stop();
}

fn run() {
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

    match solve_from_data(cli_opts.input_file, affectation_base_uri) {
        Ok(links) => {
            display_links_table(links);
        }
        Err(e) => eprint!("error: {e}")
    };

    check_final_alloc();
}
