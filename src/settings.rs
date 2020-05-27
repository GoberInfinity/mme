//TODO: Use self instead normal functions

use ansi_term::Colour;
use preferences::{prefs_base_dir, AppInfo, Preferences, PreferencesError, PreferencesMap};
use std::collections::HashMap;

const KEY: &str = "mme";
const PATH: &str = "commands_path";
const PCOLOR: &str = "text_color";
const SCOLOR: &str = "head_color";
const HCOLOR: &str = "highlight_color";

const APP_INFO: AppInfo = AppInfo {
    name: "preferences",
    author: "mme",
};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub commands_path: String,
    pub title_color: ansi_term::Colour,
    pub information_color: ansi_term::Colour,
    pub highlight_color: ansi_term::Colour,
}

impl Config {
    pub fn new(
        path: &Option<String>,
        primary_color: &Option<String>,
        secondary_color: &Option<String>,
        highlight_color: &Option<String>,
    ) -> Result<Config, &'static str> {
        let current_path = path.as_deref().unwrap_or("");
        let current_primary_color = primary_color.as_deref().unwrap_or("");
        let current_secondary_color = secondary_color.as_deref().unwrap_or("");
        let current_highlight_color = highlight_color.as_deref().unwrap_or("");

        let new_config = !current_path.is_empty()
            || !current_primary_color.is_empty()
            || !current_secondary_color.is_empty()
            || !current_highlight_color.is_empty();

        let mut current_preferences = match Config::get_preferences(
            PreferencesMap::<String>::load(&APP_INFO, &KEY),
            &current_path,
        ) {
            Ok(previous_preferences) => previous_preferences,
            Err(_) => return Err("Path not configured yet"),
        };

        let keys_val = vec![
            (PATH, current_path),
            (PCOLOR, &current_primary_color),
            (SCOLOR, &current_secondary_color),
            (HCOLOR, &current_highlight_color),
        ];

        for (key, val) in &keys_val {
            Config::change_preference_if_new(&val.to_string(), &mut current_preferences, &key);
        }

        if new_config {
            match current_preferences.save(&APP_INFO, &KEY) {
                Ok(_) => println!(
                    "Configuration saved inside: {:?}",
                    prefs_base_dir().unwrap()
                ),
                Err(_) => return Err("Cannot save your preferences"),
            }
        }

        Ok(Config {
            commands_path: String::from(current_preferences.get(keys_val[0].0).unwrap()),
            title_color: Config::get_color(current_preferences.get(keys_val[1].0).unwrap()),
            information_color: Config::get_color(current_preferences.get(keys_val[2].0).unwrap()),
            highlight_color: Config::get_color(current_preferences.get(keys_val[3].0).unwrap()),
        })
    }

    fn get_preferences(
        input: Result<HashMap<String, String>, PreferencesError>,
        current_path: &str,
    ) -> Result<HashMap<String, String>, String> {
        match input {
            // Try to find previous preferences
            Ok(user_preferences) => {
                // If it finds a preference, check that the path is in it
                if user_preferences.contains_key(&String::from("commands_path")) {
                    Ok(user_preferences)
                } else {
                    // If it doesn't find the path but has the color configurations,
                    // check that the user provided a new path
                    if current_path.is_empty() {
                        return Err("Path not configured yet, you also need a path".to_string());
                    } else {
                        Ok(user_preferences)
                    }
                }
            }
            Err(_) => {
                // If the preferences doesn't exist and the user didn't type the new path
                if current_path.is_empty() {
                    return Err("Path not configured yet".to_string());
                } else {
                    Ok(PreferencesMap::new())
                }
            }
        }
    }

    fn change_preference_if_new(new_value: &String, map: &mut HashMap<String, String>, key: &str) {
        match map.contains_key(key) {
            true => {
                // New configuration
                if !new_value.is_empty() {
                    map.insert(String::from(key), new_value.to_string());
                }
            }
            false => {
                // First time configuration with a new value
                if !new_value.is_empty() {
                    map.insert(String::from(key), new_value.to_string());
                } else {
                    map.insert(String::from(key), String::from(""));
                }
            }
        }
    }

    fn get_color(str_color: &String) -> Colour {
        match str_color.to_lowercase().as_str() {
            "black" => Colour::Black,
            "red" => Colour::Red,
            "green" => Colour::Green,
            "yellow" => Colour::Yellow,
            "blue" => Colour::Blue,
            "purple" => Colour::Purple,
            "cyan" => Colour::Cyan,
            _ => Colour::White,
        }
    }
}

#[cfg(test)]
mod test;
