/* TODO:- Add parameter to print result_of_searching
*/

use crate::user;
use ansi_term::{ANSIString, ANSIStrings, Colour};
use std::fs;
use std::path::Path;

const NAME_REP: u8 = 0;
const DESC_REP: u8 = 1;
const SEPARATOR_REP: u8 = 2;

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

    let results = search_using(word, &contents, only_by_desc, only_by_name);

    print_search_results(
        results,
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
) -> Vec<Vec<(u8, bool, &'a str)>> {
    let query = query.to_lowercase();
    let size_doc = contents.lines().count();
    let mut found = false;
    let mut last_type = NAME_SYMBOL;
    let mut result_of_searching = Vec::new();
    let mut buffer: Vec<(u8, bool, &str)> = Vec::new();

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
                    split_and_put_in_buffer(line, NAME_REP, NAME_SYMBOL, &mut buffer);
                    last_type = NAME_SYMBOL;
                }
                DESC_SYMBOL => {
                    split_and_put_in_buffer(line, DESC_REP, DESC_SYMBOL, &mut buffer);
                    last_type = DESC_SYMBOL;
                }
                _ => split_and_put_in_buffer(line, SEPARATOR_REP, SEPARATOR_SYMBOL, &mut buffer),
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
                match buffer.last() {
                    Some(line) => {
                        if line.2.to_lowercase().contains(&query) {
                            if let Some(last) = buffer.last_mut() {
                                last.1 = true;
                            }
                            found = true;
                        }
                    }
                    _ => (),
                }
            };
        }

        if empty_line || end_of_file {
            if found {
                result_of_searching.push(buffer.clone());
                found = false;
            }
            buffer.clear();
        }
    }

    result_of_searching
}

fn split_and_put_in_buffer<'a>(
    line: &'a str,
    type_symbol: u8,
    symbol_to_split: char,
    buffer: &mut Vec<(u8, bool, &'a str)>,
) {
    let information = match symbol_to_split {
        NAME_SYMBOL | DESC_SYMBOL => line.split_at(symbol_to_split.len_utf8()).1,
        _ => line,
    };

    buffer.push((type_symbol, false, information));
}

fn is_necessary_search_by_name(by_desc: &bool, by_all: &bool) -> bool {
    !(*by_desc && !*by_all)
}

fn is_necessary_search_by_desc(by_head: &bool, by_all: &bool, end_of_file: &bool) -> bool {
    //end_of_file prevents to skip the last part of the code if the iterator reach the end of the file
    !((*by_head && !by_all && *end_of_file) || (*by_head && !by_all))
}

fn print_search_results(
    results: Vec<Vec<(u8, bool, &str)>>,
    command_color: Colour,
    desc_color: Colour,
    high_color: Colour,
    word: &str,
) {
    for command in results.iter() {
        for line in command {
            let (rep, highlight, text) = line;
            let fixed_text = match rep {
                &NAME_REP => NAME,
                &DESC_REP => DESC,
                _ => "",
            };

            if !fixed_text.is_empty() {
                print_ascii_string(&[command_color.bold().paint(fixed_text)], false);
            }

            if !highlight {
                print_ascii_string(&[desc_color.paint(String::from(*text))], true);
                continue;
            }

            let mut result: Vec<ANSIString> = Vec::new();
            let mut last_inn = 0;

            for (index, matched) in text.to_lowercase().match_indices(word) {
                // text
                if last_inn != index {
                    result.push(desc_color.paint(&text[last_inn..index]));
                }
                //word
                last_inn = index + matched.len();
                result.push(high_color.bold().paint(&text[index..last_inn]));
            }
            if last_inn < text.len() {
                result.push(desc_color.paint(&text[last_inn..]));
            }

            print_ascii_string(&result, true);
        }
    }
}

fn print_ascii_string(ascii_string: &[ANSIString], separator: bool) {
    if separator {
        println!("{}{}", SEPARATOR, ANSIStrings(ascii_string));
    } else {
        println!("{}", ANSIStrings(ascii_string));
    }
}

#[cfg(test)]
#[path = "./test.rs"]
mod test;
