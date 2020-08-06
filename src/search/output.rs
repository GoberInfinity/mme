/* TODO:- Add parameter to print all
- Self fn instead of regular functions?
*/

use crate::user;
use ansi_term::{ANSIString, ANSIStrings, Colour};
use std::collections::VecDeque;
use std::fs;
use std::path::Path;

const NAME: &str = "NAME";
const DESC: &str = "DESC";
const SEPARATOR: &str = "   ";
const NAME_SYMBOL: char = '#';
const DESC_SYMBOL: char = '>';
const COMMENT_SYMBOL: char = '/';
const SEPARATOR_SYMBOL: char = '\0';

pub fn print_with_configuration(
    word: &Option<String>,
    only_by_name: &bool,
    only_by_desc: &bool,
    config: &user::preferences::Config,
) -> Result<(), &'static str> {
    let path = Path::new(&config.commands_path).to_str();
    let contents = match fs::read_to_string(path.unwrap()) {
        Ok(contents) => contents,
        Err(_) => return Err("Cannot read the file"),
    };

    let word = match word {
        Some(word) => word,
        None => return Ok(()),
    };

    let (results, mut results_indexes) = search_using(word, &contents, only_by_desc, only_by_name);

    print_search_results(
        results,
        &mut results_indexes,
        config.fixed_color,
        config.text_color,
        config.highlight_color,
        word,
    );

    Ok(())
}

fn search_using<'a>(
    query: &str,
    contents: &'a str,
    by_desc: &bool,
    by_head: &bool,
) -> (Vec<Vec<(&'a str, &'a str)>>, Vec<Vec<usize>>) {
    let query = query.to_lowercase();
    let mut found = false;
    let mut n_b: Vec<(&str, &str)> = Vec::new();
    let mut all = Vec::new();
    let size_doc = contents.lines().count();
    let mut last_type = '#';
    let mut all_found_in_element = Vec::new();
    let mut found_in_element: Vec<usize> = Vec::new();

    let by_all = match (by_head, by_desc) {
        (true, true) => true,
        (_, _) => false,
    };

    for (number_line, line) in contents.lines().enumerate() {
        let end_of_file = number_line == size_doc - 1;
        let mut empty_line = false;
        let mut chars = line.trim().chars();

        match chars.next() {
            Some(first_char) => match first_char {
                // Validate that the comment only occur in a stand alone line
                COMMENT_SYMBOL => continue,
                NAME_SYMBOL => {
                    split_and_put_in_buffer(line, NAME, NAME_SYMBOL, &mut n_b);
                    last_type = NAME_SYMBOL;
                }
                DESC_SYMBOL => {
                    split_and_put_in_buffer(line, DESC, DESC_SYMBOL, &mut n_b);
                    last_type = DESC_SYMBOL;
                }
                _ => split_and_put_in_buffer(line, SEPARATOR, SEPARATOR_SYMBOL, &mut n_b),
            },
            None => {
                empty_line = true;
            }
        }

        if !empty_line {
            if match last_type {
                NAME_SYMBOL => is_necessary_search_by_name(by_desc, &by_all),
                _ => is_necessary_search_by_desc(by_head, &by_all, &end_of_file),
            } {
                match n_b.last() {
                    Some(line) => {
                        if line.1.to_lowercase().contains(&query) {
                            found = true;
                            found_in_element.push(n_b.len() - 1);
                        }
                    }
                    _ => (),
                }
            };
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
    }

    (all, all_found_in_element)
}

fn split_and_put_in_buffer<'a>(
    line: &'a str,
    fixed_text: &'a str,
    symbol_to_split: char,
    buffer: &mut Vec<(&'a str, &'a str)>,
) {
    let information = match symbol_to_split {
        NAME_SYMBOL | DESC_SYMBOL => line.split_at(symbol_to_split.len_utf8()).1,
        _ => line,
    };

    buffer.push((fixed_text, information));
}

fn is_necessary_search_by_name(by_desc: &bool, by_all: &bool) -> bool {
    !(*by_desc && !*by_all)
}

fn is_necessary_search_by_desc(by_head: &bool, by_all: &bool, end_of_file: &bool) -> bool {
    //end_of_file prevents to skip the last part of the code if the iterator reach the end of the file
    !((*by_head && !by_all && *end_of_file) || (*by_head && !by_all))
}

fn print_search_results(
    results: Vec<Vec<(&str, &str)>>,
    indexes: &mut Vec<Vec<usize>>,
    command_color: Colour,
    desc_color: Colour,
    high_color: Colour,
    word: &str,
) {
    for (i, line) in results.iter().enumerate() {
        let current_match = &indexes[i];
        let mut last = 0;
        for (j, current) in line.iter().enumerate() {
            let (title, info) = current;

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

            if !(current_match.len() == last) && j == *current_match.get(last).unwrap() {
                let mut result: Vec<ANSIString> = Vec::new();
                let mut last_inn = 0;
                let lower_case_line = info.to_lowercase();

                for (index, matched) in lower_case_line.match_indices(word) {
                    // text
                    if last_inn != index {
                        let res = &info[last_inn..index];
                        let word = desc_color.paint(res);
                        result.push(word);
                    }
                    //word
                    last_inn = index + matched.len();
                    let orignal_word = &info[index..last_inn];
                    let word = high_color.bold().paint(orignal_word);
                    result.push(word);
                }
                if last_inn < info.len() {
                    let res = &info[last_inn..];
                    let word = desc_color.paint(res);
                    result.push(word);
                }
                last += 1;

                let strings: &[ANSIString] = &result;
                println!("  {}", ANSIStrings(strings));
            } else {
                let strings: &[ANSIString<'static>] = &[desc_color.paint(String::from(*info))];
                println!("  {}", ANSIStrings(strings));
            }
        }
    }
}

#[cfg(test)]
#[path = "./test.rs"]
mod test;
