use clap::{command, Arg, ArgMatches};
use std::process;

// messer /home/user/.config -s "source text" -r "replacement text" -i

const DIRECTORY_ARG: &str = "directory";
const SOURCE_TEXT_ARG: &str = "source-text";
const REPLACEMENT_TEXT_ARG: &str = "replacement-text";
const CASE_INSENSITIVE_ARG: &str = "case-insensitive";

#[derive(Debug)]
pub struct Config<'a> {
    pub directory: &'a std::path::PathBuf,
    pub source_text: &'a str,
    pub replacement_text: &'a str,
    pub case_insensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(matches: &'a clap::ArgMatches) -> Config<'a> {
        let directory = matches
            .get_one::<std::path::PathBuf>(DIRECTORY_ARG).unwrap();
        let source_text = matches
            .get_one::<String>(SOURCE_TEXT_ARG).unwrap();
        let replacement_text = matches
            .get_one::<String>(REPLACEMENT_TEXT_ARG).unwrap();
        let case_insensitive: bool = matches
            .get_flag(CASE_INSENSITIVE_ARG);
        
        let config = Config {
            directory,
            source_text,
            replacement_text,
            case_insensitive,
        };

        config.validate();

        config
    }

    pub fn validate(&self) {
        if !self.directory.exists() {
            println!("Error: path `{}` doesn't exist", self.directory.display());
            process::exit(1);
        }
    
        if !self.directory.is_dir() {
            println!("Error: `{}` is not a directory", self.directory.display());
            process::exit(1);
        }
    }
}

pub fn matches() -> ArgMatches {
    command!()
        .about("Utility for replacing all occurrences of source text with replacement text in all files from provided directory")
        .arg(
            Arg::new(DIRECTORY_ARG)
                .required(true)
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .help("Directory with text files to be edited")
        )
        .arg(
            Arg::new(SOURCE_TEXT_ARG)
                .short('s')
                .help("Source text to search for")
                .required(true)
        )
        .arg(
            Arg::new(REPLACEMENT_TEXT_ARG)
                .short('r')
                .help("Replacement text")
                .required(true)
        )
        .arg(
            Arg::new(CASE_INSENSITIVE_ARG)
                .short('i')
                .help("Perform a case-insensitive search")
                .num_args(0)
        )
        .get_matches()
}
