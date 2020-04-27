use crate::input;
use crate::settings;
use ansi_term::{ANSIString, ANSIStrings};
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn print_with_configuration(
    user_input: &input::Command,
    config: &settings::Config,
) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&config.commands_path).to_str();

    let contents = fs::read_to_string(path.unwrap())?;
    let mut word = &String::from("aaa");
    match &user_input.word_to_search {
        Some(val) => word = val,
        None => (),
    }
    let results = search(word, &contents); //TODO:Check if the word is empty

    for line in results.iter() {
        for stri in line {
            match *stri {
                "NAME" | "DESC" => {
                    let strings: &[ANSIString<'static>] =
                        &[config.title_color.bold().paint(String::from(*stri))];
                    println!("{}", ANSIStrings(strings));
                }
                _ => {
                    let strings: &[ANSIString<'static>] =
                        &[config.information_color.paint(String::from(*stri))];
                    println!("  {}", ANSIStrings(strings));
                }
            }
        }
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<VecDeque<&'a str>> {
    let query = query.to_lowercase();
    let mut is_final = false;
    let mut found = false;

    let mut buffer_lines: VecDeque<&str> = VecDeque::new();
    let mut all = Vec::new();

    let size_doc = contents.lines().count();

    for (i, line) in contents.lines().enumerate() {
        buffer_lines.push_back(line);

        if line.len() == 0 {
            is_final = true;
            buffer_lines.pop_back();
        }

        if i == size_doc - 1 {
            is_final = true;
        }

        if line.to_lowercase().contains(&query) {
            found = true;
        }

        if is_final && found {
            all.push(buffer_lines.clone());
            found = false;
        }

        if is_final {
            is_final = false;
            buffer_lines.clear();
        }
    }

    all
}
