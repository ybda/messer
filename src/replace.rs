use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use crate::config::Config;
use regex;

fn replace_case_insensitive(source: &str, find: &str, replace: &str) -> String {
    // Create a case-insensitive regex pattern
    let regex_pattern = format!(r"(?i){}", find);
    let regex = regex::Regex::new(&regex_pattern).expect("Invalid regex pattern");

    // Use the regex to replace occurrences in the source string
    let result = regex.replace_all(source, replace);

    result.into()
}

fn replace_text_in_file(file_path: &Path, source_text: &str, replacement_text: &str, case_insensitive: bool) -> io::Result<()> {
    let mut content = String::new();
    let mut file = File::open(file_path)?;

    file.read_to_string(&mut content)?;

    let replaced_content = if case_insensitive {
        replace_case_insensitive(&content, source_text, replacement_text)
    } else {
        content.replace(source_text, replacement_text)
    };

    let mut file = File::create(file_path)?;
    file.write_all(replaced_content.as_bytes())?;

    Ok(())
}

fn process_file(config: &Config, file_path: &Path) -> io::Result<()> {
    replace_text_in_file(file_path, config.source_text, config.replacement_text, config.case_insensitive)
}

pub fn replace_text_in_files(config: &Config) -> io::Result<()> {
    for entry in WalkDir::new(&config.directory).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            process_file(config, entry.path())?;
        }
    }

    Ok(())
}