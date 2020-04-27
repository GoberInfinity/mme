use crate::input;
use crate::settings;
use ansi_term::{ANSIString, ANSIStrings};
use std::collections::VecDeque;
use std::fs;
use std::path::Path;

// TODO: - Improve the way that you print the information to the user, maybe an private struct
//  - Debug command line with name in multiple lines

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
        for stri in line {
            let first_char = stri.chars().next();
            let mut skip = false;
            let mut text = String::new();

            match first_char {
                Some('#') => {
                    let strings: &[ANSIString<'static>] =
                        &[config.title_color.bold().paint(String::from("NAME"))];
                    println!("{}", ANSIStrings(strings));
                    skip = true;
                }

                Some('>') => {
                    let strings: &[ANSIString<'static>] =
                        &[config.title_color.bold().paint(String::from("DESC"))];
                    println!("{}", ANSIStrings(strings));
                    skip = true;
                }
                _ => (),
            }

            if skip {
                let (_, info) = stri.split_at('#'.len_utf8());
                text = String::from(info);
            } else {
                text = String::from(*stri);
            }

            let strings: &[ANSIString<'static>] = &[config.information_color.paint(text)];
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
) -> Vec<VecDeque<&'a str>> {
    let query = query.to_lowercase();
    let mut found = false;
    let mut buffer_lines: VecDeque<&str> = VecDeque::new();
    let mut all = Vec::new();
    let size_doc = contents.lines().count();

    let by_all = match (by_head, by_desc) {
        (true, true) => true,
        (_, _) => false,
    };

    for (i, line) in contents.lines().enumerate() {
        let empty_line = line.trim().is_empty();

        if !empty_line {
            buffer_lines.push_back(line);
        }

        let first_char = line.chars().next();

        match first_char {
            Some(first_char) => match first_char {
                '#' if *by_desc && !by_all => continue,
                '>' if *by_head && !by_all => continue,
                _ => (),
            },
            None => (), // Empty line
        }

        if line.to_lowercase().contains(&query) {
            found = true;
        }

        if empty_line || i == size_doc - 1 {
            if found {
                all.push(buffer_lines.clone());
                found = false;
            }
            buffer_lines.clear();
            continue;
        }
    }

    all
}
