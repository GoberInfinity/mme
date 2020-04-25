use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Command {
    /// Word to search in the document specified in the configuration
    pub word_to_search: Option<String>,

    /// Searches the word only in names
    #[structopt(short = "n", long = "name")]
    pub search_only_in_name: bool,

    /// Searches the word only in descriptions
    #[structopt(short = "d", long = "desc")]
    pub search_only_in_desc: bool,

    #[structopt(long, parse(from_os_str))]
    pub path: Option<PathBuf>,

    #[structopt(long)]
    pub primary_color: Option<String>,

    #[structopt(long)]
    pub secondary_color: Option<String>,
}
/*
pub fn get_cli_arguments(user_input: Command) {
    // You should validate the path if there is a path
    // You should validate first and secondary color
    let mut u_word = String::new();
    let mut u_path = PathBuf::new();
    let mut u_p_color = String::new();
    let mut u_s_color = String::new();

    match user_input.word_to_search {
        Some(word_to_search) => u_word = word_to_search,
        None => (),
    }

    match user_input.path {
        Some(path) => u_path = path,
        None => (),
    }

    match user_input.primary_color {
        Some(primary_color) => u_p_color = primary_color,
        None => println!(),
    }

    match user_input.secondary_color {
        Some(secondary_color) => u_s_color = secondary_color,
        None => println!(),
    }

    return [u_word, path, u_p_color, u_s_color];
}*/
