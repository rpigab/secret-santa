use env_logger::Env;

use secret_santa_core::run;

fn main() {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "always");

    env_logger::init_from_env(env);

    match run() {
        Ok(_) => {},
        Err(e) => eprint!("error: {e}")
    }
}
