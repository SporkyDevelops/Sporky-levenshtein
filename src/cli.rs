use clap::Parser;

#[derive(Parser)]
#[clap(author = "Sporky Develops", version = "0.1", about = "Really crappy levenshtein spell checker", long_about = None)]
pub struct Cli {
    pub target: Option<String>,
    #[clap(
        short = 'v',
        long = "verbose",
        default_value_t = false,
        help = "Prints long output",
        long_help = "Changes wether or not score/distance is visible."
    )]
    pub verbose: bool,
    #[clap(
        short = 'n',
        long = "number",
        default_value_t = 5,
        help = "Change the number of matches shown",
        long_help = "The number of words the program will show. The default value is 5."
    )]
    pub number: usize,
}