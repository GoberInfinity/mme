use structopt::StructOpt;

#[derive(Debug, StructOpt)]
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

fn add(one: i32) -> i32 {
    one + one
}

//#[path = "tests/uinput.rs"]
#[cfg(test)]
mod test;
