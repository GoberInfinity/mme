use super::*;
use std::io::{Error, ErrorKind};

const INPUT_BLUE_HCOLOR: &str = "blue";
const INPUT_GREEN_HCOLOR: &str = "green";

const INPUT_BLUE_TCOLOR: &str = "blue";
const INPUT_GREEN_TCOLOR: &str = "green";

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
    preferences_with_colors.insert(HCOLOR.to_string(), INPUT_BLUE_HCOLOR.to_string());
    assert_eq!(
        Config::get_preferences(Ok(preferences_with_colors), ""),
        Err("Path not configured yet, you also need a path".to_string())
    );
}

#[test]
fn change_all_configurations() {
    let mut all_preferences: HashMap<String, String> = HashMap::new();
    all_preferences.insert(PATH.to_string(), INPUT_ROOT_SPATH.to_string());
    all_preferences.insert(HCOLOR.to_string(), INPUT_BLUE_HCOLOR.to_string());
    all_preferences.insert(TCOLOR.to_string(), INPUT_BLUE_TCOLOR.to_string());
    Config::change_preference_if_new(&INPUT_HOME_SPATH.to_string(), &mut all_preferences, PATH);
    Config::change_preference_if_new(
        &INPUT_GREEN_HCOLOR.to_string(),
        &mut all_preferences,
        HCOLOR,
    );
    Config::change_preference_if_new(
        &INPUT_GREEN_TCOLOR.to_string(),
        &mut all_preferences,
        TCOLOR,
    );

    assert_eq!(
        all_preferences.get(PATH),
        Some(&INPUT_HOME_SPATH.to_string())
    );
    assert_eq!(
        all_preferences.get(HCOLOR),
        Some(&INPUT_GREEN_HCOLOR.to_string())
    );
    assert_eq!(
        all_preferences.get(TCOLOR),
        Some(&INPUT_GREEN_TCOLOR.to_string())
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
