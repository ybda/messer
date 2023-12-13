use clap::{command, Arg, ArgMatches};
use std::env;
use std::fs::File;
use std::path::PathBuf;

const DIRECTORY_ARG: &str = "directory";
const SOURCE_TEXT_ARG: &str = "source-text";
const REPLACEMENT_TEXT_ARG: &str = "replacement-text";
const VERBOSE_ARG: &str = "verbose";
const CASE_INSENSITIVE_ARG: &str = "case-insensitive";
const INTERACTIVE_ARG: &str = "interactive";

#[derive(Debug)]
pub struct Config<'a> {
    pub directory: &'a PathBuf,
    pub source_text: &'a str,
    pub replacement_text: &'a str,
    pub verbose: bool,
    pub case_insensitive: bool,
    pub interactive: bool,
}

impl<'a> Config<'a> {
    pub fn new(matches: &'a clap::ArgMatches) -> Result<Config<'a>, String> {
        let config = Config {
            directory: matches.get_one::<PathBuf>(DIRECTORY_ARG).unwrap(),
            source_text: matches.get_one::<String>(SOURCE_TEXT_ARG).unwrap(),
            replacement_text: matches.get_one::<String>(REPLACEMENT_TEXT_ARG).unwrap(),
            verbose: matches.get_flag(VERBOSE_ARG),
            case_insensitive: matches.get_flag(CASE_INSENSITIVE_ARG),
            interactive: matches.get_flag(INTERACTIVE_ARG),
        };

        config.validate()?;

        Ok(config)
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.directory.exists() {
            return Err("Path does not exist");
        }

        if !self.directory.is_dir() {
            return Err("Path is not a directory");
        }

        Ok(())
    }
}

pub fn matches() -> ArgMatches {
    command!()
        .about("Utility for replacing all occurrences of source text with replacement text in all files from provided directory")
        .arg(
            Arg::new(DIRECTORY_ARG)
                .required(true)
                .value_parser(clap::value_parser!(PathBuf))
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
            Arg::new(VERBOSE_ARG)
                .short('v')
                .long("verbose")
                .help("Print name of each changed file")
                .num_args(0)
        )
        .arg(
            Arg::new(CASE_INSENSITIVE_ARG)
                .short('i')
                .long("ignore-case")
                .help("Perform a case-insensitive search")
                .num_args(0)
        )
        .arg(
            Arg::new(INTERACTIVE_ARG)
                .short('I')
                .long("interactive")
                .help("Prompt before every file change")
                .num_args(0)
        )
        .get_matches()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_path_doesnt_exist_validation() {
        let temp_dir = env::temp_dir();

        let path_that_doesnt_exist = temp_dir.join("messer_path_that_doesnt_exist_032563575713");

        let config: Config = Config {
            directory: &path_that_doesnt_exist,
            source_text: "_",
            replacement_text: "_",
            verbose: false,
            case_insensitive: false,
            interactive: false,
        };

        assert_eq!(config.validate(), Err("Path does not exist"));
    }

    #[test]
    fn test_path_is_not_dir() {
        let temp_dir: PathBuf = env::temp_dir();

        let test_file_path: PathBuf = temp_dir.join("messer_temp_test_file");

        File::create(&test_file_path).unwrap_or_else(|e| panic!("Error: {}", e));

        let config: Config = Config {
            directory: &test_file_path,
            source_text: "_",
            replacement_text: "_",
            verbose: false,
            case_insensitive: false,
            interactive: false,
        };

        let validate_result = config.validate();

        fs::remove_file(&test_file_path).unwrap_or_else(|e| panic!("Error: {}", e));

        assert_eq!(validate_result, Err("Path is not a directory"));
    }
}
