use super::*;

pub const BINARY: &str = "mme";
pub const PATH: &str = "--path";
pub const INPUT_PATH: &str = "../examples/c";
pub const PRIMARY: &str = "--primary-color";
pub const INPUT_PRIMARY: &str = "black";
pub const SECONDARY: &str = "--secondary-color";
pub const INPUT_SECONDARY: &str = "white";
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
        Command::from_iter(&[BINARY, NAME, DESC, INPUT_WORD]),
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
        Command::from_iter(&[BINARY, NAME, "--", INPUT_SPECIAL_WORD]),
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
