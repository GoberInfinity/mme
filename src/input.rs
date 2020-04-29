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

    #[structopt(long)]
    pub path: Option<String>,

    #[structopt(long)]
    pub primary_color: Option<String>,

    #[structopt(long)]
    pub secondary_color: Option<String>,
}

#[cfg(test)]
mod test;
