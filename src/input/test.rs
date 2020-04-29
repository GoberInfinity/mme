use super::*;

#[test]
fn no_arguments() {
    assert_eq!(
        no_arguments_command(),
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
        all_arguments_command(),
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
        all_config_command(),
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
        all_search_command(),
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
        special_search_with_parameters_command(),
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
