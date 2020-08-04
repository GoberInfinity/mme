// TODO - Instead of sending user_input only pass the required parameters to the function

mod parser;
mod search;
mod user;
use std::process;
use structopt::StructOpt;

fn main() {
    let user_input = parser::input::Command::from_args();

    let config = user::preferences::Config::new(
        &user_input.path,
        &user_input.fixed_color,
        &user_input.text_color,
        &user_input.highlight_color,
    )
    .unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = search::output::print_with_configuration(
        &user_input.word_to_search,
        &user_input.search_only_in_name,
        &user_input.search_only_in_desc,
        &config,
    ) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
