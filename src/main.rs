use mme::Config;

use std::env;
use std::process;

fn main() {
    //let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //Run function doesn’t return a value that we want to unwrap
    if let Err(e) = mme::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
