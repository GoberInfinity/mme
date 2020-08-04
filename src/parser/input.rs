use structopt::StructOpt;
// TODO:  Add documentation
// Add unit  testing

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

    #[structopt(short = "p", long)]
    pub path: Option<String>,

    #[structopt(short = "f", long)]
    pub fixed_color: Option<String>,

    #[structopt(short = "t", long)]
    pub text_color: Option<String>,

    #[structopt(short = "h", long)]
    pub highlight_color: Option<String>,
}

#[cfg(test)]
#[path = "./test.rs"]
mod test;
