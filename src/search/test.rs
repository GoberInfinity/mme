use super::*;

// TODO: Add unit testing to print all

const SINGLE_FILE_COMMAND: &str = "#ls -la -- # > *
>Lists directory > # *
";

const FILE_COMMAND: &str = "#ls -la -- # > *
>Lists directory > # *

#rm -rf -all
>Deletes all --

#rustup default <version>
> It uses
the rust version
specified

#ssh
-h
-a
>Creates
secure
connection";

const SINGLE_FILE_COMMAND_UTF8: &str = "#ls -la -- # > *
>Каталог списков > # *";

const SPECIAL_COMMAND: &str = "--";
const NON_EXISTENT_COMMAND: &str = "whoami";

const LS_COMMAND: &str = "ls";
const LS_DESC: &str = "Lists";
const LS_DESC_UTF8: &str = "Каталог";

const N_RESULT_LS_2: &str = "ls -la -- # > *";
const D_RESULT_LS_2: &str = "Lists directory > # *";
const D_RESULT_UTF8_LS_2: &str = "Каталог списков > # *";

const RM_COMMAND: &str = "rm";
const RM_DESC: &str = "Deletes all --";

const N_RESULT_RM_2: &str = "rm -rf -all";
const D_RESULT_RM_2: &str = "Deletes all --";

const RUST_COMMAND: &str = "rustup";

const N_RESULT_RUST_2: &str = "rustup default <version>";
const D_RESULT_RUST_2: &str = "It uses";
const RESULT_RUST_1_2: &str = "the rust version";
const RESULT_RUST_2_2: &str = "specified";

const MULTIPLE_LINE_COMMAND: &str = "-h";
const MULTIPLE_LINE_DESC: &str = "secure";

const N_RESULT_ML_1_2: &str = "ssh";
const N_RESULT_ML_2_2: &str = "-h";
const N_RESULT_ML_3_2: &str = "-a";
const D_RESULT_ML_1_2: &str = "Creates";
const D_RESULT_ML_2_2: &str = "secure";
const D_RESULT_ML_3_2: &str = "connection";

#[test]
fn command_by_all_word_in_name_file_with_one_command() {
    let expected_result: Vec<Vec<(u8, bool, &str)>> =
        vec![vec![(0, true, N_RESULT_LS_2), (1, false, D_RESULT_LS_2)]
            .into_iter()
            .collect()];
    assert_eq!(
        search_using(LS_COMMAND, SINGLE_FILE_COMMAND, &false, &false, &false),
        expected_result
    );
}

#[test]
fn command_by_all_word_in_desc_file_with_one_command() {
    let expected_result: Vec<Vec<(u8, bool, &str)>> =
        vec![vec![(0, false, N_RESULT_LS_2), (1, true, D_RESULT_LS_2)]
            .into_iter()
            .collect()];

    assert_eq!(
        search_using(LS_DESC, SINGLE_FILE_COMMAND, &false, &false, &false),
        expected_result
    );
}

#[test]
fn utf8_command_by_all_word_in_name_file_with_one_command() {
    let expected_result: Vec<Vec<(u8, bool, &str)>> = vec![vec![
        (0, true, N_RESULT_LS_2),
        (1, false, D_RESULT_UTF8_LS_2),
    ]
    .into_iter()
    .collect()];

    assert_eq!(
        search_using(LS_COMMAND, SINGLE_FILE_COMMAND_UTF8, &false, &false, &false),
        expected_result
    );
}

#[test]
fn utf8_command_by_all_word_in_desc_file_with_one_command() {
    let expected_result: Vec<Vec<(u8, bool, &str)>> = vec![vec![
        (0, false, N_RESULT_LS_2),
        (1, true, D_RESULT_UTF8_LS_2),
    ]
    .into_iter()
    .collect()];

    assert_eq!(
        search_using(
            LS_DESC_UTF8,
            SINGLE_FILE_COMMAND_UTF8,
            &false,
            &false,
            &false
        ),
        expected_result
    );
}

#[test]
fn command_by_name_word_in_name_file() {
    let expected_result: Vec<Vec<(u8, bool, &str)>> =
        vec![vec![(0, true, N_RESULT_RM_2), (1, false, D_RESULT_RM_2)]
            .into_iter()
            .collect()];

    assert_eq!(
        search_using(RM_COMMAND, FILE_COMMAND, &false, &false, &true),
        expected_result
    );
}

#[test]
fn command_by_name_word_in_desc_file() {
    let expected_result: Vec<Vec<(u8, bool, &str)>> =
        vec![vec![(0, false, N_RESULT_RM_2), (1, true, D_RESULT_RM_2)]
            .into_iter()
            .collect()];

    assert_eq!(
        search_using(RM_DESC, FILE_COMMAND, &false, &true, &false),
        expected_result
    );
}

#[test]
fn command_by_name_and_desc_word_in_name_file() {
    let expected_result: Vec<Vec<(u8, bool, &str)>> = vec![vec![
        (0, true, N_RESULT_RUST_2),
        (1, false, D_RESULT_RUST_2),
        (2, false, RESULT_RUST_1_2),
        (2, false, RESULT_RUST_2_2),
    ]
    .into_iter()
    .collect()];

    assert_eq!(
        search_using(RUST_COMMAND, FILE_COMMAND, &false, &true, &true),
        expected_result
    );
}

#[test]
fn special_command_by_name_word_in_desc_file() {
    let expected_result: Vec<Vec<(u8, bool, &str)>> =
        vec![vec![(0, true, N_RESULT_LS_2), (1, false, D_RESULT_LS_2)]
            .into_iter()
            .collect()];

    assert_eq!(
        search_using(SPECIAL_COMMAND, FILE_COMMAND, &false, &false, &true),
        expected_result
    );
}

#[test]
fn special_command_by_desc_word_in_desc_file() {
    let expected_result: Vec<Vec<(u8, bool, &str)>> =
        vec![vec![(0, false, N_RESULT_RM_2), (1, true, D_RESULT_RM_2)]
            .into_iter()
            .collect()];

    assert_eq!(
        search_using(SPECIAL_COMMAND, FILE_COMMAND, &false, &true, &false),
        expected_result
    );
}

#[test]
fn command_by_name_word_nowhere_file() {
    let expected_result: Vec<Vec<(u8, bool, &str)>> = vec![];
    assert_eq!(
        search_using(NON_EXISTENT_COMMAND, FILE_COMMAND, &false, &false, &true),
        expected_result
    );
}

#[test]
fn command_by_desc_word_nowhere_file() {
    let expected_result: Vec<Vec<(u8, bool, &str)>> = vec![];
    assert_eq!(
        search_using(NON_EXISTENT_COMMAND, FILE_COMMAND, &false, &true, &false),
        expected_result
    );
}

#[test]
fn command_by_all_word_in_multiple_desc_file() {
    let expected_result: Vec<Vec<(u8, bool, &str)>> = vec![vec![
        (0, false, N_RESULT_ML_1_2),
        (2, false, N_RESULT_ML_2_2),
        (2, false, N_RESULT_ML_3_2),
        (1, false, D_RESULT_ML_1_2),
        (2, true, D_RESULT_ML_2_2),
        (2, false, D_RESULT_ML_3_2),
    ]
    .into_iter()
    .collect()];

    assert_eq!(
        search_using(MULTIPLE_LINE_DESC, FILE_COMMAND, &false, &false, &false),
        expected_result
    );
}

#[test]
fn command_by_name_word_in_multiple_file() {
    let expected_result: Vec<Vec<(u8, bool, &str)>> = vec![vec![
        (0, false, N_RESULT_ML_1_2),
        (2, true, N_RESULT_ML_2_2),
        (2, false, N_RESULT_ML_3_2),
        (1, false, D_RESULT_ML_1_2),
        (2, false, D_RESULT_ML_2_2),
        (2, false, D_RESULT_ML_3_2),
    ]
    .into_iter()
    .collect()];
    assert_eq!(
        search_using(MULTIPLE_LINE_COMMAND, FILE_COMMAND, &false, &false, &true),
        expected_result
    );
}

#[test]
fn command_by_desc_word_in_multiple_file() {
    let expected_result: Vec<Vec<(u8, bool, &str)>> = vec![vec![
        (0, false, N_RESULT_ML_1_2),
        (2, false, N_RESULT_ML_2_2),
        (2, false, N_RESULT_ML_3_2),
        (1, false, D_RESULT_ML_1_2),
        (2, true, D_RESULT_ML_2_2),
        (2, false, D_RESULT_ML_3_2),
    ]
    .into_iter()
    .collect()];

    assert_eq!(
        search_using(MULTIPLE_LINE_DESC, FILE_COMMAND, &false, &true, &false),
        expected_result
    );
}
