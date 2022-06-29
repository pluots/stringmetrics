use stringmetrics::{algorithms, spellcheck};

use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Everything related to spellchecking
    Check {
        /// Path to a dictionary file. Specify e.g. dictionaries/de_DE if
        /// dictionaries/de_DE.aff and dictionaries/de_DE.dic exist
        #[clap(short, long, value_parser)]
        dict_path: String,

        /// Whether to print misspelled words
        #[clap(short, long, value_parser, default_value_t = false)]
        list_misspelled: bool,
    },
    /// Levenshtein distance tools
    Lev {
        /// The start string to calculate distance from
        #[clap(value_parser)]
        string_a: String,

        /// The end string to calculate distance to
        #[clap(value_parser)]
        string_b: String,

        /// Specify a maximum difference limit for the levenshthein distance
        #[clap(short, long, value_parser, default_value_t = 1000)]
        limit: u32,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Check {
            dict_path,
            list_misspelled,
        } => {
            println!("{:?}", dict_path);
            println!("{:?}", list_misspelled);
        }
        Commands::Lev {
            string_a,
            string_b,
            limit,
        } => {
            println!(
                "{}",
                algorithms::levenshtein_limit(string_a, string_b, *limit)
            )
        } // None => {}
    }
}
