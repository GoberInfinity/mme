use structopt::StructOpt;

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

    /// Prints all the commands and their descriptions
    #[structopt(short = "a", long = "all")]
    pub print_all: bool,

    /// Sets the path of the file where mme is going to search
    #[structopt(short = "p", long)]
    pub path: Option<String>,

    ///Sets the color of NAME, DESC words
    #[structopt(short = "f", long)]
    pub fixed_color: Option<String>,

    /// Sets the color of the text
    #[structopt(short = "t", long)]
    pub text_color: Option<String>,

    ///Sets the color of the highlighted word
    #[structopt(short = "h", long)]
    pub highlight_color: Option<String>,
}

#[cfg(test)]
#[path = "./test.rs"]
mod test;
