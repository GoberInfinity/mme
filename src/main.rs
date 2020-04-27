mod input;
mod search;
mod settings;
use std::process;
use structopt::StructOpt;

fn main() {
    let user_input = input::Command::from_args();

    let config = settings::Config::new(&user_input).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = search::print_with_configuration(&user_input, &config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
