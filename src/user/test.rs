use super::*;
use std::io::{Error, ErrorKind};

const INPUT_BLUE_FIXED_COLOR: &str = "blue";
const INPUT_GREEN_FIXED_COLOR: &str = "green";
const INPUT_WHITE_TEXT_COLOR: &str = "white";
const INPUT_BLACK_TEXT_COLOR: &str = "black";
const INPUT_YELLOW_HIGH_COLOR: &str = "yellow";
const INPUT_CYAN_HIGH_COLOR: &str = "cyan";
const INPUT_HOME_PATH: &str = "~";
const INPUT_ROOT_PATH: &str = "/";

#[test]
fn no_previous_preferences_no_path() {
    assert_eq!(
        Config::get_preferences(
            Err(PreferencesError::Io(Error::new(ErrorKind::Other, "oh no!"))),
            ""
        ),
        Err("Path not configured yet".to_string())
    );
}

#[test]
fn no_previous_preferences_no_path_but_colors() {
    let mut preferences_with_colors: HashMap<String, String> = HashMap::new();
    preferences_with_colors.insert(FIXED_COLOR.to_string(), INPUT_BLUE_FIXED_COLOR.to_string());
    assert_eq!(
        Config::get_preferences(Ok(preferences_with_colors), ""),
        Err("Path not configured yet, you also need a path".to_string())
    );
}

#[test]
fn change_all_configurations() {
    let mut all_preferences: HashMap<String, String> = HashMap::new();
    all_preferences.insert(PATH.to_string(), INPUT_ROOT_PATH.to_string());
    all_preferences.insert(FIXED_COLOR.to_string(), INPUT_BLUE_FIXED_COLOR.to_string());
    all_preferences.insert(TEXT_COLOR.to_string(), INPUT_WHITE_TEXT_COLOR.to_string());
    all_preferences.insert(HIGH_COLOR.to_string(), INPUT_YELLOW_HIGH_COLOR.to_string());

    Config::change_preference_if_new(&INPUT_HOME_PATH.to_string(), &mut all_preferences, PATH);

    Config::change_preference_if_new(
        &INPUT_GREEN_FIXED_COLOR.to_string(),
        &mut all_preferences,
        FIXED_COLOR,
    );

    Config::change_preference_if_new(
        &INPUT_BLACK_TEXT_COLOR.to_string(),
        &mut all_preferences,
        TEXT_COLOR,
    );

    Config::change_preference_if_new(
        &INPUT_CYAN_HIGH_COLOR.to_string(),
        &mut all_preferences,
        HIGH_COLOR,
    );

    assert_eq!(
        all_preferences.get(PATH),
        Some(&INPUT_HOME_PATH.to_string())
    );
    assert_eq!(
        all_preferences.get(FIXED_COLOR),
        Some(&INPUT_GREEN_FIXED_COLOR.to_string())
    );
    assert_eq!(
        all_preferences.get(TEXT_COLOR),
        Some(&INPUT_BLACK_TEXT_COLOR.to_string())
    );
    assert_eq!(
        all_preferences.get(HIGH_COLOR),
        Some(&INPUT_CYAN_HIGH_COLOR.to_string())
    );
}

#[test]
fn change_path_configuration_by_creation() {
    let mut path_preferences: HashMap<String, String> = HashMap::new();
    Config::change_preference_if_new(&INPUT_HOME_PATH.to_string(), &mut path_preferences, PATH);
    assert_eq!(
        path_preferences.get(PATH),
        Some(&INPUT_HOME_PATH.to_string())
    );
}
