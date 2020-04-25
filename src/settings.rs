use crate::input;
use ansi_term::Colour;
use ansi_term::{ANSIString, ANSIStrings};
use std::collections::HashMap;
use std::path::PathBuf;

extern crate preferences;
use preferences::{AppInfo, Preferences, PreferencesMap};

const APP_INFO: AppInfo = AppInfo {
    name: "preferences",
    author: "mme",
};
const KEY: &str = "config";

pub struct Config {
    pub commands_path: String, //Change to StringPath
    pub title_color: ansi_term::Colour,
    pub information_color: ansi_term::Colour,
}

impl Config {
    pub fn new(user_input: input::Command) -> Result<Config, &'static str> {
        let mut new_path = PathBuf::new();
        let mut new_primary_color = String::new();
        let mut new_secondary_color = String::new();

        match user_input.path {
            Some(path) => new_path = path,
            None => (),
        }

        match user_input.primary_color {
            Some(primary_color) => new_primary_color = primary_color,
            None => (),
        }

        match user_input.secondary_color {
            Some(secondary_color) => new_secondary_color = secondary_color,
            None => (),
        }

        if !new_path.as_os_str().is_empty()
            || !new_primary_color.is_empty()
            || !new_secondary_color.is_empty()
        {
            change_configuration(new_path, new_primary_color, new_secondary_color)
        }

        Ok(Config {
            commands_path: String::from("aaa"),
            title_color: Colour::White,
            information_color: Colour::White,
        })
    }
}

fn change_configuration(path: PathBuf, primary_color: String, secondary_color: String) {
    let mut user_configurations = read_to_change_configuration();
    user_configurations.insert(
        String::from("command_path"),
        path.as_path().display().to_string(), //Note, we should check the path before assign it here
    );
    println!("{:?}", user_configurations);
    update_configuration(user_configurations);
}

fn read_to_change_configuration() -> PreferencesMap {
    match PreferencesMap::<String>::load(&APP_INFO, &KEY) {
        Ok(preferences) => return preferences,
        Err(_) => return PreferencesMap::new(),
    }
}

fn update_configuration(updated_configuration: PreferencesMap) {
    match updated_configuration.save(&APP_INFO, &KEY) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
