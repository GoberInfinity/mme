use ansi_term::Colour::Red;
use ansi_term::Style;
use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = match env::var("MME_CFS") {
            Ok(filename) => filename,
            Err(_) => return Err("MME_CFS enviroment path not found"),
        };
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = search(&config.query, &contents);

    for (i, line) in results.iter().enumerate() {
        for stri in line {
            match *stri {
                "NAME" => print!("{}", Red.paint(*stri)),
                "DESC" => {
                    println!("",);
                    print!("{}", Red.paint(*stri));
                }
                _ => {
                    println!("",);
                    print!("     {}", Style::new().italic().paint(*stri));
                }
            }
        }
        if i == results.len() - 1 {
            continue;
        } else {
            println!("",);
            println!("",);
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
