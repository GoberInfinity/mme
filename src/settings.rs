/* TODO
    - Check that the file exists
    - Check that the color option matches
*/

use crate::input;
use ansi_term::Colour;
use std::path::PathBuf;

extern crate preferences;
use preferences::{prefs_base_dir, AppInfo, Preferences, PreferencesMap};

const APP_INFO: AppInfo = AppInfo {
    name: "preferences",
    author: "mme",
};
const KEY: &str = "mme";

pub struct Config {
    pub commands_path: PathBuf,
    pub title_color: ansi_term::Colour,
    pub information_color: ansi_term::Colour,
}

impl Config {
    pub fn new(user_input: input::Command) -> Result<Config, &'static str> {
        let mut current_path = PathBuf::new();
        let mut current_primary_color = String::new();
        let mut current_secondary_color = String::new();
        let mut new_config = false;

        match user_input.path {
            Some(path) => current_path = path,
            None => (),
        }

        match user_input.primary_color {
            Some(primary_color) => current_primary_color = primary_color,
            None => (),
        }

        match user_input.secondary_color {
            Some(secondary_color) => current_secondary_color = secondary_color,
            None => (),
        }

        if !current_path.as_os_str().is_empty()
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
                    if current_path.as_os_str().is_empty() {
                        return Err("Path not configured yet inside");
                    } else {
                        user_preferences
                    }
                }
            }
            Err(_) => {
                // If the preferences doesn't exist and the user didn't type the new path
                if current_path.as_os_str().is_empty() {
                    return Err("Path not configured yet");
                } else {
                    PreferencesMap::new()
                }
            }
        };

        let mut path_match = &String::from("commands_path");
        let mut fc_match = &String::from("head_color");
        let mut sc_match = &String::from("text_color");

        if new_config {
            user_preferences.insert(
                path_match.to_string(),
                current_path.clone().into_os_string().into_string().unwrap(),
            );
            user_preferences.insert(fc_match.to_string(), current_primary_color);
            user_preferences.insert(sc_match.to_string(), current_secondary_color);
            match user_preferences.save(&APP_INFO, &KEY) {
                Ok(_) => println!("Configuration saved inside: {:?}", prefs_base_dir()),
                Err(_) => return Err("Cannot save your preferences"),
            }
        }

        /*
            // Assign the new values to the configuration
            for (key, val) in user_preferences.iter() {
                match (key, val) {
                    (path_match, val) if current_path.as_os_str().is_empty() => {
                        current_path = PathBuf::from(val)
                    }
                    (fc_match, val) if current_primary_color.is_empty() => {
                        current_primary_color = String::from(val)
                    }
                    (sc_match, val) if current_secondary_color.is_empty() => {
                        current_secondary_color = String::from(val)
                    }
                    _ => (),
                }
            }
        */

        Ok(Config {
            commands_path: current_path,
            title_color: Colour::White,
            information_color: Colour::White,
        })
    }

    fn get_color(str_color: String) -> Colour {
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
}
