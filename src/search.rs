/* TODO:- Add parameter to print all
*/

use crate::input;
use crate::settings;
use ansi_term::Colour::Red;
use ansi_term::{ANSIString, ANSIStrings, Colour};
use std::collections::VecDeque;
use std::fs;
use std::path::Path;

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

    let (results, mut results_indexes) = search_using(
        word,
        &contents,
        &user_input.search_only_in_desc,
        &user_input.search_only_in_name,
    );

    print_search_results(
        results,
        &mut results_indexes,
        config.title_color,
        config.information_color,
    );

    Ok(())
}

fn search_using<'a>(
    query: &str,
    contents: &'a str,
    by_desc: &bool,
    by_head: &bool,
) -> (Vec<VecDeque<(&'a str, &'a str)>>, Vec<VecDeque<usize>>) {
    let query = query.to_lowercase();
    let mut found = false;
    let mut n_b: VecDeque<(&str, &str)> = VecDeque::new();
    let mut all = Vec::new();
    let size_doc = contents.lines().count();
    let mut last_type = '#';
    let mut search = true;

    let mut all_found_in_element = Vec::new();
    let mut found_in_element: VecDeque<usize> = VecDeque::new();

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
                        last_type = '#';
                        if *by_desc && !by_all {
                            search = false;
                        }
                    }
                    '>' => {
                        let (_, info) = line.split_at('>'.len_utf8());
                        n_b.push_back(("DESC", info));
                        last_type = '>';
                        //end_of_file prevents to skip the last part of the code if the iterator reach the end of the file
                        if *by_head && !by_all && end_of_file {
                            search = false;
                        } else if *by_head && !by_all {
                            search = false;
                        }
                    }
                    _ => match last_type {
                        '#' => {
                            n_b.push_back(("   ", line));
                            if *by_desc && !by_all {
                                search = false;
                            }
                        }
                        '>' => {
                            n_b.push_back(("   ", line));
                            if *by_head && !by_all && end_of_file {
                                search = false;
                            } else if *by_head && !by_all {
                                search = false;
                            }
                        }
                        _ => (),
                    },
                },
                None => (),
            }

            if search {
                match n_b.back() {
                    Some(line) => {
                        if line.1.to_lowercase().contains(&query) {
                            found = true;
                            found_in_element.push_back(n_b.len() - 1);
                        }
                    }
                    _ => (),
                }
            }
        }

        if empty_line || end_of_file {
            if found {
                all.push(n_b.clone());
                all_found_in_element.push(found_in_element.clone());
                found = false;
            }
            last_type = '#';
            n_b.clear();
            found_in_element.clear();
        }

        search = true;
    }

    (all, all_found_in_element)
}

fn print_search_results(
    results: Vec<VecDeque<(&str, &str)>>,
    indexes: &mut Vec<VecDeque<usize>>,
    command_color: Colour,
    desc_color: Colour,
) {
    for (i, line) in results.iter().enumerate() {
        let current_match = &indexes[i];
        let mut last = 0;
        for (j, current) in line.iter().enumerate() {
            let (title, info) = current;

            if !(current_match.len() == last) && j == *current_match.get(last).unwrap() {
                let mut result: Vec<ANSIString> = Vec::new();
                let mut last_inn = 0;

                for (index, matched) in info.match_indices("ls") {
                    // word
                    if last_inn != index {
                        let res = &info[last_inn..index];
                        let word = desc_color.paint(res);
                        result.push(word);
                    }
                    //special character
                    let word = Red.bold().paint(matched);
                    result.push(word);
                    last_inn = index + matched.len();
                }
                if last_inn < info.len() {
                    let res = &info[last_inn..];
                    let word = desc_color.bold().paint(res);
                    result.push(word);
                }
                //println!("{:?}", result);
                last += 1;

                let strings: &[ANSIString] = &result;
                println!("  {}", ANSIStrings(strings));
            }

            match title {
                &"NAME" => {
                    let strings: &[ANSIString<'static>] =
                        &[command_color.bold().paint(String::from("NAME"))];
                    println!("{}", ANSIStrings(strings));
                }

                &"DESC" => {
                    let strings: &[ANSIString<'static>] =
                        &[command_color.bold().paint(String::from("DESC"))];
                    println!("{}", ANSIStrings(strings));
                }
                _ => (),
            }

            let strings: &[ANSIString<'static>] = &[desc_color.paint(String::from(*info))];
            println!("  {}", ANSIStrings(strings));
        }
    }
}

#[cfg(test)]
mod test;
