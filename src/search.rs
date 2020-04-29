use crate::input;
use crate::settings;
use ansi_term::{ANSIString, ANSIStrings};
use std::collections::VecDeque;
use std::fs;
use std::path::Path;

/* TODO: - Bug when passing # or > (it prints all) -> search using mme -- "mme -ls ##"
        - Check the support to non-ascii strings
*/

pub fn print_with_configuration(
    user_input: &input::Command,
    config: &settings::Config,
) -> Result<(), &'static str> {
    let path = Path::new(&config.commands_path).to_str();
    let contents = match fs::read_to_string(path.unwrap()) {
        Ok(contents) => contents,
        Err(_) => return Err("Cannot read the file"),
    };

    let word = match &user_input.word_to_search {
        Some(word) => word,
        None => return Ok(()),
    };

    let results = search(
        word,
        &contents,
        &user_input.search_only_in_desc,
        &user_input.search_only_in_name,
    );

    for line in results.iter() {
        for (title, info) in line {
            match title {
                &"NAME" => {
                    let strings: &[ANSIString<'static>] =
                        &[config.title_color.bold().paint(String::from("NAME"))];
                    println!("{}", ANSIStrings(strings));
                }

                &"DESC" => {
                    let strings: &[ANSIString<'static>] =
                        &[config.title_color.bold().paint(String::from("DESC"))];
                    println!("{}", ANSIStrings(strings));
                }
                _ => (),
            }

            let strings: &[ANSIString<'static>] =
                &[config.information_color.paint(String::from(*info))];
            println!("  {}", ANSIStrings(strings));
        }
    }

    Ok(())
}

fn search<'a>(
    query: &str,
    contents: &'a str,
    by_desc: &bool,
    by_head: &bool,
) -> Vec<VecDeque<(&'a str, &'a str)>> {
    let query = query.to_lowercase();
    let mut found = false;
    let mut n_b: VecDeque<(&str, &str)> = VecDeque::new();
    let mut all = Vec::new();
    let size_doc = contents.lines().count();

    let by_all = match (by_head, by_desc) {
        (true, true) => true,
        (_, _) => false,
    };

    for (i, line) in contents.lines().enumerate() {
        let empty_line = line.trim().is_empty();
        let end_of_file = i == size_doc - 1;

        if !empty_line {
            match line.chars().next() {
                Some(first_char) => match first_char {
                    '#' => {
                        //Divide one string slice into two at an index.
                        let (_, info) = line.split_at('#'.len_utf8());
                        n_b.push_back(("NAME", info));
                        if *by_desc && !by_all {
                            continue;
                        }
                    }
                    '>' => {
                        let (_, info) = line.split_at('>'.len_utf8());
                        n_b.push_back(("DESC", info));
                        if *by_head && !by_all && !end_of_file {
                            continue;
                        }
                    }
                    _ => n_b.push_back(("   ", line)),
                },
                None => (),
            }
        }

        match n_b.back() {
            Some(line) => {
                if line.1.to_lowercase().contains(&query) {
                    found = true;
                }
            }
            _ => (),
        }

        if empty_line || end_of_file {
            if found {
                all.push(n_b.clone());
                found = false;
            }
            n_b.clear();
        }
    }

    all
}
