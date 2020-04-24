use mme::Command;
use mme::Config;
use std::env;
use std::process;
use structopt::StructOpt;

fn main() {
    let command_options = Command::from_args();

    let config = Config::new(command_options).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = mme::run(config, env::args()) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
