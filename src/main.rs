use mme::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = mme::run(config, env::args()) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
