use clap::Parser;
use env_logger::Env;

use secret_santa_cli::display_links_table;
use secret_santa_core::solve::benchmark_solve;
use secret_santa_utils::bench;
use secret_santa_utils::chrono::Chrono;

#[derive(Parser)]
struct CliOpts {
    num_nodes: usize,
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

    match benchmark_solve(cli_opts.num_nodes) {
        Ok(links) => {
            display_links_table(links, DEFAULT_AFFECTATION_BASE_URI.to_string());
        }
        Err(e) => log::error!("{e}")
    };

    bench::alloc::check_final_alloc();
}
