use ansi_term::Colour;
use ansi_term::{ANSIString, ANSIStrings};
use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::FromIterator;
use std::path::Path;

pub struct Config {
    pub commands_path: String,
    pub title_color: ansi_term::Colour,
    pub information_color: ansi_term::Colour,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let filename = match env::var("MME_CF") {
            Ok(filename) => filename,
            Err(_) => return Err("MME_CF enviroment path not found"),
        };

        let file = File::open(Path::new(&filename)).or_else(|_| {
            return Err("Commands file not found");
        });

        let unwrapped_file = &file.unwrap();
        let content = BufReader::new(unwrapped_file);
        let mut commands_path: String = String::new();
        let mut bold_color: Colour = Colour::White;
        let mut text_color: Colour = Colour::White;

        for line in content.lines() {
            let line = line.unwrap();
            let t_line = line.trim();
            if t_line.is_empty() {
                continue;
            }

            let tokens = Vec::from_iter(t_line.split_whitespace());
            let name = tokens.first().unwrap();
            let value: String = tokens.get(1).unwrap().to_string();

            match name.to_uppercase().as_str() {
                "COMMANDSPATH" => commands_path = value,
                "BOLDCOLOR" => bold_color = get_color(&value),
                "TEXTCOLOR" => text_color = get_color(&value),
                _ => (),
            }
        }

        Ok(Config {
            commands_path: String::from(commands_path),
            title_color: bold_color,
            information_color: text_color,
        })
    }
}

pub fn get_color(value: &str) -> Colour {
    let bold_color = match value.to_ascii_lowercase().as_ref() {
        "black" => Colour::Cyan,
        "red" => Colour::Red,
        "green" => Colour::Green,
        "yellow" => Colour::Yellow,
        "blue" => Colour::Blue,
        "purple" => Colour::Purple,
        "cyan" => Colour::Cyan,
        _ => Colour::White,
    };
    bold_color
}

pub fn run(config: Config, mut args: std::env::Args) -> Result<(), Box<dyn Error>> {
    args.next(); // Skip name of the program

    let query = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a query string")?,
    };

    let path = Path::new(&config.commands_path).to_str();
    let contents = fs::read_to_string(path.unwrap())?;
    let results = search(&query, &contents);

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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<VecDeque<&'a str>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_command() {
        let not_in_query = "not";
        let empty_command: Vec<VecDeque<&str>> = Vec::new();
        let simple_contents = "NAME
command
DESC
Description";

        assert_eq!(empty_command, search(not_in_query, simple_contents));
    }

    #[test]
    fn simple_command() {
        let simple_query = "command";
        let simple_contents = "NAME
command
DESC
Description";

        let mut simple_command = Vec::new();
        let mut simple_lines: VecDeque<&str> = VecDeque::new();
        simple_lines.push_back(&"NAME");
        simple_lines.push_back(&"command");
        simple_lines.push_back(&"DESC");
        simple_lines.push_back(&"Description");
        simple_command.push(simple_lines);

        assert_eq!(simple_command, search(simple_query, simple_contents));
    }

    #[test]
    fn multiple_command() {
        let simple_query = "command";
        let simple_contents = "NAME
command
DESC
Description

NAME
command
DESC
Description

NAME
command
DESC
Description";

        let mut simple_command = Vec::new();
        let mut simple_lines: VecDeque<&str> = VecDeque::new();
        simple_lines.push_back(&"NAME");
        simple_lines.push_back(&"command");
        simple_lines.push_back(&"DESC");
        simple_lines.push_back(&"Description");
        simple_command.push(simple_lines.clone());
        simple_command.push(simple_lines.clone());
        simple_command.push(simple_lines);

        assert_eq!(simple_command, search(simple_query, simple_contents));
    }

}
