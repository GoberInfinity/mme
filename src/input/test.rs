use super::*;

use structopt::StructOpt;
const BINARY: &str = "mme";
const PATH: &str = "--path";
const INPUT_PATH: &str = "file";
const PRIMARY: &str = "--primary-color";
const INPUT_PRIMARY: &str = "black";
const SECONDARY: &str = "--secondary-color";
const INPUT_SECONDARY: &str = "white";
const NAME: &str = "-n";
const DESC: &str = "-d";
const INPUT_WORD: &str = "word";
const INPUT_SPECIAL_WORD: &str = "-word";

#[test]
fn no_arguments() {
    assert_eq!(
        Command::from_args(),
        Command {
            word_to_search: None,
            search_only_in_name: false,
            search_only_in_desc: false,
            path: None,
            primary_color: None,
            secondary_color: None,
        }
    );
}

#[test]
fn all_arguments() {
    // The first argument will be parsed as the binary name
    assert_eq!(
        Command::from_iter(&[
            BINARY,
            PATH,
            INPUT_PATH,
            PRIMARY,
            INPUT_PRIMARY,
            SECONDARY,
            INPUT_SECONDARY,
            NAME,
            DESC,
            INPUT_WORD,
        ]),
        Command {
            word_to_search: Some(INPUT_WORD.to_string()),
            search_only_in_name: true,
            search_only_in_desc: true,
            path: Some(INPUT_PATH.to_string()),
            primary_color: Some(INPUT_PRIMARY.to_string()),
            secondary_color: Some(INPUT_SECONDARY.to_string()),
        }
    );
}

#[test]
fn all_config() {
    assert_eq!(
        Command::from_iter(&[
            BINARY,
            PATH,
            INPUT_PATH,
            PRIMARY,
            INPUT_PRIMARY,
            SECONDARY,
            INPUT_SECONDARY,
        ]),
        Command {
            word_to_search: None,
            search_only_in_name: false,
            search_only_in_desc: false,
            path: Some(INPUT_PATH.to_string()),
            primary_color: Some(INPUT_PRIMARY.to_string()),
            secondary_color: Some(INPUT_SECONDARY.to_string()),
        }
    );
}

#[test]
fn all_search() {
    assert_eq!(
        Command::from_iter(&[BINARY, NAME, DESC, INPUT_WORD,]),
        Command {
            word_to_search: Some(INPUT_WORD.to_string()),
            search_only_in_name: true,
            search_only_in_desc: true,
            path: None,
            primary_color: None,
            secondary_color: None,
        }
    );
}

#[test]
fn special_search_with_parameters() {
    assert_eq!(
        Command::from_iter(&[BINARY, NAME, "--", INPUT_SPECIAL_WORD,]),
        Command {
            word_to_search: Some(INPUT_SPECIAL_WORD.to_string()),
            search_only_in_name: true,
            search_only_in_desc: false,
            path: None,
            primary_color: None,
            secondary_color: None,
        }
    );
}
