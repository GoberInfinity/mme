use super::*;

const SINGLE_FILE_COMMAND: &str = "#ls -la -- # > *
>Lists directory > # *
";

const FILE_COMMAND: &str = "#ls -la -- # > *
>Lists directory > # *

#rm -rf -all
>Deletes all --

#rustup default <version>
> It use
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
const N_RESULT_LS: (&str, &str) = ("NAME", "ls -la -- # > *");
const D_RESULT_LS: (&str, &str) = ("DESC", "Lists directory > # *");
const D_RESULT_UTF8_LS: (&str, &str) = ("DESC", "Каталог списков > # *");

const RM_COMMAND: &str = "rm";
const RM_DESC: &str = "Deletes all --";
const N_RESULT_RM: (&str, &str) = ("NAME", "rm -rf -all");
const D_RESULT_RM: (&str, &str) = ("DESC", "Deletes all --");

const RUST_COMMAND: &str = "rustup";
const N_RESULT_RUST: (&str, &str) = ("NAME", "rustup default <version>");
const D_RESULT_RUST: (&str, &str) = ("DESC", " It use");
const RESULT_RUST_1: (&str, &str) = ("   ", "the rust version");
const RESULT_RUST_2: (&str, &str) = ("   ", "specified");

const MULTIPLE_LINE_COMMAND: &str = "-h";
const MULTIPLE_LINE_DESC: &str = "secure";
const N_RESULT_ML_1: (&str, &str) = ("NAME", "ssh");
const N_RESULT_ML_2: (&str, &str) = ("   ", "-h");
const N_RESULT_ML_3: (&str, &str) = ("   ", "-a");
const D_RESULT_ML_1: (&str, &str) = ("DESC", "Creates");
const D_RESULT_ML_2: (&str, &str) = ("   ", "secure");
const D_RESULT_ML_3: (&str, &str) = ("   ", "connection");

#[test]
fn command_by_name_in_file_with_one_command() {
    let expected_result: Vec<VecDeque<(&str, &str)>> =
        vec![vec![N_RESULT_LS, D_RESULT_LS].into_iter().collect()];
    assert_eq!(
        search_using(LS_COMMAND, SINGLE_FILE_COMMAND, &false, &false),
        expected_result
    );
}

#[test]
fn command_by_desc_in_file_with_one_command() {
    let expected_result: Vec<VecDeque<(&str, &str)>> =
        vec![vec![N_RESULT_LS, D_RESULT_LS].into_iter().collect()];
    assert_eq!(
        search_using(LS_DESC, SINGLE_FILE_COMMAND, &false, &false),
        expected_result
    );
}

#[test]
fn utf8_command_by_name_in_file_with_one_command() {
    let expected_result: Vec<VecDeque<(&str, &str)>> =
        vec![vec![N_RESULT_LS, D_RESULT_UTF8_LS].into_iter().collect()];
    assert_eq!(
        search_using(LS_COMMAND, SINGLE_FILE_COMMAND_UTF8, &false, &false),
        expected_result
    );
}

#[test]
fn utf8_command_by_desc_in_file_with_one_command() {
    let expected_result: Vec<VecDeque<(&str, &str)>> =
        vec![vec![N_RESULT_LS, D_RESULT_UTF8_LS].into_iter().collect()];
    assert_eq!(
        search_using(LS_DESC_UTF8, SINGLE_FILE_COMMAND_UTF8, &false, &false),
        expected_result
    );
}

#[test]
fn command_by_name_in_file() {
    let expected_result: Vec<VecDeque<(&str, &str)>> =
        vec![vec![N_RESULT_RM, D_RESULT_RM].into_iter().collect()];
    assert_eq!(
        search_using(RM_COMMAND, FILE_COMMAND, &false, &true),
        expected_result
    );
}

#[test]
fn command_by_desc_in_file() {
    let expected_result: Vec<VecDeque<(&str, &str)>> =
        vec![vec![N_RESULT_RM, D_RESULT_RM].into_iter().collect()];
    assert_eq!(
        search_using(RM_DESC, FILE_COMMAND, &true, &false),
        expected_result
    );
}

#[test]
fn command_by_name_and_desc_in_file() {
    let expected_result: Vec<VecDeque<(&str, &str)>> =
        vec![
            vec![N_RESULT_RUST, D_RESULT_RUST, RESULT_RUST_1, RESULT_RUST_2]
                .into_iter()
                .collect(),
        ];
    assert_eq!(
        search_using(RUST_COMMAND, FILE_COMMAND, &true, &true),
        expected_result
    );
}

#[test]
fn special_command_by_name_in_file() {
    let expected_result: Vec<VecDeque<(&str, &str)>> =
        vec![vec![N_RESULT_LS, D_RESULT_LS].into_iter().collect()];
    assert_eq!(
        search_using(SPECIAL_COMMAND, FILE_COMMAND, &false, &true),
        expected_result
    );
}

#[test]
fn special_command_by_desc_in_file() {
    let expected_result: Vec<VecDeque<(&str, &str)>> =
        vec![vec![N_RESULT_RM, D_RESULT_RM].into_iter().collect()];
    assert_eq!(
        search_using(SPECIAL_COMMAND, FILE_COMMAND, &true, &false),
        expected_result
    );
}

#[test]
fn command_by_name_not_in_file() {
    let expected_result: Vec<VecDeque<(&str, &str)>> = vec![];
    assert_eq!(
        search_using(NON_EXISTENT_COMMAND, FILE_COMMAND, &false, &true),
        expected_result
    );
}

#[test]
fn command_by_desc_not_in_file() {
    let expected_result: Vec<VecDeque<(&str, &str)>> = vec![];
    assert_eq!(
        search_using(NON_EXISTENT_COMMAND, FILE_COMMAND, &true, &false),
        expected_result
    );
}

#[test]
fn command_in_multiple_lines_in_file() {
    let expected_result: Vec<VecDeque<(&str, &str)>> = vec![vec![
        N_RESULT_ML_1,
        N_RESULT_ML_2,
        N_RESULT_ML_3,
        D_RESULT_ML_1,
        D_RESULT_ML_2,
        D_RESULT_ML_3,
    ]
    .into_iter()
    .collect()];
    assert_eq!(
        search_using(MULTIPLE_LINE_DESC, FILE_COMMAND, &false, &false),
        expected_result
    );
}

#[test]
fn command_in_multiple_lines_by_name_in_file() {
    let expected_result: Vec<VecDeque<(&str, &str)>> = vec![vec![
        N_RESULT_ML_1,
        N_RESULT_ML_2,
        N_RESULT_ML_3,
        D_RESULT_ML_1,
        D_RESULT_ML_2,
        D_RESULT_ML_3,
    ]
    .into_iter()
    .collect()];
    assert_eq!(
        search_using(MULTIPLE_LINE_COMMAND, FILE_COMMAND, &false, &true),
        expected_result
    );
}

#[test]
fn command_in_multiple_lines_by_desc_in_file() {
    let expected_result: Vec<VecDeque<(&str, &str)>> = vec![vec![
        N_RESULT_ML_1,
        N_RESULT_ML_2,
        N_RESULT_ML_3,
        D_RESULT_ML_1,
        D_RESULT_ML_2,
        D_RESULT_ML_3,
    ]
    .into_iter()
    .collect()];
    assert_eq!(
        search_using(MULTIPLE_LINE_DESC, FILE_COMMAND, &true, &false),
        expected_result
    );
}
