use super::*;

// TODO: -Create the unit testing for print all

pub const BINARY: &str = "mme";
pub const PATH: &str = "--path";
pub const INPUT_PATH: &str = "../examples/c";
pub const FIXED: &str = "--fixed-color";
pub const INPUT_FIXED: &str = "black";
pub const TEXT: &str = "--text-color";
pub const INPUT_TEXT: &str = "white";
pub const HIGHLIGHT: &str = "--highlight-color";
pub const INPUT_HIGHLIGHT: &str = "blue";
pub const ALL: &str = "-a";
pub const NAME: &str = "-n";
pub const DESC: &str = "-d";
pub const INPUT_WORD: &str = "word";
pub const INPUT_SPECIAL_WORD: &str = "-word";

#[test]
fn no_arguments() {
    assert_eq!(
        Command::from_args(),
        Command {
            word_to_search: None,
            print_all: false,
            search_only_in_name: false,
            search_only_in_desc: false,
            path: None,
            fixed_color: None,
            text_color: None,
            highlight_color: None,
        }
    );
}

#[test]
fn all_arguments() {
    assert_eq!(
        Command::from_iter(&[
            BINARY,
            PATH,
            INPUT_PATH,
            FIXED,
            INPUT_FIXED,
            TEXT,
            INPUT_TEXT,
            HIGHLIGHT,
            INPUT_HIGHLIGHT,
            ALL,
            NAME,
            DESC,
            INPUT_WORD,
        ]),
        Command {
            word_to_search: Some(INPUT_WORD.to_string()),
            print_all: true,
            search_only_in_name: true,
            search_only_in_desc: true,
            path: Some(INPUT_PATH.to_string()),
            fixed_color: Some(INPUT_FIXED.to_string()),
            text_color: Some(INPUT_TEXT.to_string()),
            highlight_color: Some(INPUT_HIGHLIGHT.to_string()),
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
            FIXED,
            INPUT_FIXED,
            TEXT,
            INPUT_TEXT,
            HIGHLIGHT,
            INPUT_HIGHLIGHT,
        ]),
        Command {
            word_to_search: None,
            print_all: false,
            search_only_in_name: false,
            search_only_in_desc: false,
            path: Some(INPUT_PATH.to_string()),
            fixed_color: Some(INPUT_FIXED.to_string()),
            text_color: Some(INPUT_TEXT.to_string()),
            highlight_color: Some(INPUT_HIGHLIGHT.to_string()),
        }
    );
}

#[test]
fn all_search() {
    assert_eq!(
        Command::from_iter(&[BINARY, ALL, NAME, DESC, INPUT_WORD]),
        Command {
            word_to_search: Some(INPUT_WORD.to_string()),
            print_all: true,
            search_only_in_name: true,
            search_only_in_desc: true,
            path: None,
            fixed_color: None,
            text_color: None,
            highlight_color: None,
        }
    );
}

#[test]
fn special_search_with_parameters() {
    assert_eq!(
        Command::from_iter(&[BINARY, NAME, "--", INPUT_SPECIAL_WORD]),
        Command {
            word_to_search: Some(INPUT_SPECIAL_WORD.to_string()),
            print_all: false,
            search_only_in_name: true,
            search_only_in_desc: false,
            path: None,
            fixed_color: None,
            text_color: None,
            highlight_color: None,
        }
    );
}
