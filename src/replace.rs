use crate::config::Config;
use crate::error::Result;
use crate::util;
use regex::{self, Regex};
use std::borrow::Cow;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use walkdir::WalkDir;

fn replace_case_insensitive(source: &str, find: &str, replace: &str) -> String {
    // Create a case-insensitive regex pattern
    let regex_pattern: String = format!(r"(?i){}", find);
    let regex: Regex = regex::Regex::new(&regex_pattern).expect("Invalid regex pattern");

    // Use the regex to replace occurrences in the source string
    let result: Cow<'_, str> = regex.replace_all(source, replace);

    result.into()
}

fn process_file(config: &Config, file_path: &Path) -> Result<()> {
    if config.interactive {
        let prompt: String = format!("messer: change file '{}'", file_path.to_str().unwrap());
        if !util::prompt_user(prompt.as_str(), false) {
            return Ok(());
        }
    }

    let mut file = File::open(file_path)?;

    let mut content: String = String::new();
    file.read_to_string(&mut content)?;

    let new_content: String = if config.case_insensitive {
        replace_case_insensitive(&content, config.source_text, config.replacement_text)
    } else {
        content.replace(config.source_text, config.replacement_text)
    };

    if content != new_content {
        let mut file: File = File::create(file_path)?;
        file.write_all(new_content.as_bytes())?;

        if config.verbose {
            println!("messer: changed '{}'", file_path.to_str().unwrap());
        }
    }

    Ok(())
}

pub fn replace_text_in_files(config: &Config) -> Result<()> {
    for entry in WalkDir::new(&config.directory)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            process_file(config, entry.path())?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_case_insensitive() {
        let find = "cAm";
        let replace = "QWER";
        let source = "Lorem Camum docAmrc\namsitcAM cam CAMamet";
        let expected = "Lorem QWERum doQWERrc\namsitQWER QWER QWERamet";

        let result = replace_case_insensitive(source, find, replace);

        assert_eq!(result, expected.to_string());
    }
}
