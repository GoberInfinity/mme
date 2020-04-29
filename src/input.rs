use structopt::StructOpt;

pub const BINARY: &str = "mme";
pub const PATH: &str = "--path";
pub const INPUT_PATH: &str = "file";
pub const PRIMARY: &str = "--primary-color";
pub const INPUT_PRIMARY: &str = "black";
pub const SECONDARY: &str = "--secondary-color";
pub const INPUT_SECONDARY: &str = "white";
pub const NAME: &str = "-n";
pub const DESC: &str = "-d";
pub const INPUT_WORD: &str = "word";
pub const INPUT_SPECIAL_WORD: &str = "-word";

#[derive(Debug, PartialEq, StructOpt)]
pub struct Command {
    /// Word to search in the document specified in the configuration
    pub word_to_search: Option<String>,

    /// Searches the word only in names
    #[structopt(short = "n", long = "name")]
    pub search_only_in_name: bool,

    /// Searches the word only in descriptions
    #[structopt(short = "d", long = "desc")]
    pub search_only_in_desc: bool,

    #[structopt(long)]
    pub path: Option<String>,

    #[structopt(long)]
    pub primary_color: Option<String>,

    #[structopt(long)]
    pub secondary_color: Option<String>,
}

pub fn no_arguments_command() -> Command {
    Command::from_args()
}

pub fn all_arguments_command() -> Command {
    Command::from_iter(&[
        BINARY,
        PATH,
        INPUT_PATH,
        PRIMARY,
        INPUT_PRIMARY,
        SECONDARY,
        INPUT_SECONDARY,
        NAME,
        DESC,
        INPUT_WORD,
    ])
}

pub fn all_config_command() -> Command {
    Command::from_iter(&[
        BINARY,
        PATH,
        INPUT_PATH,
        PRIMARY,
        INPUT_PRIMARY,
        SECONDARY,
        INPUT_SECONDARY,
    ])
}

pub fn all_search_command() -> Command {
    Command::from_iter(&[BINARY, NAME, DESC, INPUT_WORD])
}

pub fn special_search_with_parameters_command() -> Command {
    Command::from_iter(&[BINARY, NAME, "--", INPUT_SPECIAL_WORD])
}

#[cfg(test)]
mod test;
