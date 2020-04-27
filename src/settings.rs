/* TODO
    - Change string to PathBuf
    - Refactor function change_preference_if_new to iterate instead of changing individually
    - Decide what happens when the user want to change the configuration and no word is provided
        - mme _ => nothing mme -a => means print all
*/

use crate::input;
use ansi_term::Colour;
use std::collections::HashMap;
extern crate preferences;
use preferences::{prefs_base_dir, AppInfo, Preferences, PreferencesMap};

const APP_INFO: AppInfo = AppInfo {
    name: "preferences",
    author: "mme",
};
const KEY: &str = "mme";

#[derive(Debug)]
pub struct Config {
    pub commands_path: String,
    pub title_color: ansi_term::Colour,
    pub information_color: ansi_term::Colour,
}

impl Config {
    pub fn new(user_input: &input::Command) -> Result<Config, &'static str> {
        let mut current_path = String::new();
        let mut current_primary_color = String::new();
        let mut current_secondary_color = String::new();
        let mut new_config = false;

        match &user_input.path {
            Some(path) => current_path = path.to_string(),
            None => (),
        }

        match &user_input.primary_color {
            Some(primary_color) => current_primary_color = primary_color.to_string(),
            None => (),
        }

        match &user_input.secondary_color {
            Some(secondary_color) => current_secondary_color = secondary_color.to_string(),
            None => (),
        }

        if !current_path.is_empty()
            || !current_primary_color.is_empty()
            || !current_secondary_color.is_empty()
        {
            new_config = true;
        }

        let mut user_preferences = match PreferencesMap::<String>::load(&APP_INFO, &KEY) {
            // Try to find previous preferences
            Ok(user_preferences) => {
                // If it finds a preference, check that the path is in it
                if user_preferences.contains_key(&String::from("commands_path")) {
                    user_preferences
                } else {
                    // If it doesn't find the path but has the color configurations,
                    // check that the user provided a new path
                    if current_path.is_empty() {
                        return Err("Path not configured yet inside");
                    } else {
                        user_preferences
                    }
                }
            }
            Err(_) => {
                // If the preferences doesn't exist and the user didn't type the new path
                if current_path.is_empty() {
                    return Err("Path not configured yet");
                } else {
                    PreferencesMap::new()
                }
            }
        };

        let keys = vec![
            String::from("commands_path"),
            String::from("head_color"),
            String::from("text_color"),
        ];

        Config::change_preference_if_new(current_path, &mut user_preferences, &keys[0]);
        Config::change_preference_if_new(current_primary_color, &mut user_preferences, &keys[1]);
        Config::change_preference_if_new(current_secondary_color, &mut user_preferences, &keys[2]);

        if new_config {
            match user_preferences.save(&APP_INFO, &KEY) {
                Ok(_) => println!("Configuration saved inside: {:?}", prefs_base_dir()),
                Err(_) => return Err("Cannot save your preferences"),
            }
        }

        Ok(Config {
            commands_path: String::from(user_preferences.get(&keys[0]).unwrap()),
            title_color: Config::get_color(user_preferences.get(&keys[1]).unwrap()),
            information_color: Config::get_color(user_preferences.get(&keys[2]).unwrap()),
        })
    }

    fn get_color(str_color: &String) -> Colour {
        match str_color.to_lowercase().as_str() {
            "black" => Colour::Cyan,
            "red" => Colour::Red,
            "green" => Colour::Green,
            "yellow" => Colour::Yellow,
            "blue" => Colour::Blue,
            "purple" => Colour::Purple,
            "cyan" => Colour::Cyan,
            _ => Colour::White,
        }
    }

    fn change_preference_if_new(
        new_value: String,
        map: &mut HashMap<String, String>,
        key: &String,
    ) {
        match map.contains_key(key) {
            true => {
                // New configuration
                if !new_value.is_empty() {
                    map.insert(String::from(key), new_value);
                }
            }
            false => {
                // First time configuration with a new value
                if !new_value.is_empty() {
                    map.insert(String::from(key), new_value);
                } else {
                    map.insert(String::from(key), String::from(""));
                }
            }
        }
    }
}
