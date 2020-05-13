use super::*;
use std::io::{Error, ErrorKind};

const INPUT_BLUE_PCOLOR: &str = "blue";
const INPUT_GREEN_PCOLOR: &str = "green";
const INPUT_WHITE_SCOLOR: &str = "white";
const INPUT_BLACK_SCOLOR: &str = "black";
const INPUT_YELLOW_HCOLOR: &str = "yellow";
const INPUT_CYAN_HCOLOR: &str = "cyan";
const INPUT_HOME_SPATH: &str = "~";
const INPUT_ROOT_SPATH: &str = "/";

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
    preferences_with_colors.insert(PCOLOR.to_string(), INPUT_BLUE_PCOLOR.to_string());
    assert_eq!(
        Config::get_preferences(Ok(preferences_with_colors), ""),
        Err("Path not configured yet, you also need a path".to_string())
    );
}

#[test]
fn change_all_configurations() {
    let mut all_preferences: HashMap<String, String> = HashMap::new();
    all_preferences.insert(PATH.to_string(), INPUT_ROOT_SPATH.to_string());
    all_preferences.insert(PCOLOR.to_string(), INPUT_BLUE_PCOLOR.to_string());
    all_preferences.insert(SCOLOR.to_string(), INPUT_WHITE_SCOLOR.to_string());
    all_preferences.insert(HCOLOR.to_string(), INPUT_YELLOW_HCOLOR.to_string());

    Config::change_preference_if_new(&INPUT_HOME_SPATH.to_string(), &mut all_preferences, PATH);

    Config::change_preference_if_new(
        &INPUT_GREEN_PCOLOR.to_string(),
        &mut all_preferences,
        PCOLOR,
    );

    Config::change_preference_if_new(
        &INPUT_BLACK_SCOLOR.to_string(),
        &mut all_preferences,
        SCOLOR,
    );

    Config::change_preference_if_new(&INPUT_CYAN_HCOLOR.to_string(), &mut all_preferences, HCOLOR);

    assert_eq!(
        all_preferences.get(PATH),
        Some(&INPUT_HOME_SPATH.to_string())
    );
    assert_eq!(
        all_preferences.get(PCOLOR),
        Some(&INPUT_GREEN_PCOLOR.to_string())
    );
    assert_eq!(
        all_preferences.get(SCOLOR),
        Some(&INPUT_BLACK_SCOLOR.to_string())
    );
    assert_eq!(
        all_preferences.get(HCOLOR),
        Some(&INPUT_CYAN_HCOLOR.to_string())
    );
}

#[test]
fn change_path_configuration_by_creation() {
    let mut path_preferences: HashMap<String, String> = HashMap::new();
    Config::change_preference_if_new(&INPUT_HOME_SPATH.to_string(), &mut path_preferences, PATH);
    assert_eq!(
        path_preferences.get(PATH),
        Some(&INPUT_HOME_SPATH.to_string())
    );
}
