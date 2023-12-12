use crate::config::Config;
use regex;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use walkdir::WalkDir;

fn replace_case_insensitive(source: &str, find: &str, replace: &str) -> String {
    // Create a case-insensitive regex pattern
    let regex_pattern = format!(r"(?i){}", find);
    let regex = regex::Regex::new(&regex_pattern).expect("Invalid regex pattern");

    // Use the regex to replace occurrences in the source string
    let result = regex.replace_all(source, replace);

    result.into()
}

fn process_file(config: &Config, file_path: &Path) -> io::Result<()> {
    let mut content = String::new();
    let mut file = File::open(file_path)?;

    file.read_to_string(&mut content)?;

    let replaced_content = if config.case_insensitive {
        replace_case_insensitive(&content, config.source_text, config.replacement_text)
    } else {
        content.replace(config.source_text, config.replacement_text)
    };

    if content != replaced_content {
        let mut file = File::create(file_path)?;
        file.write_all(replaced_content.as_bytes())?;

        if config.verbose {
            println!("Changed file: {}", file_path.to_str().unwrap());
        }
    }

    Ok(())
}

pub fn replace_text_in_files(config: &Config) -> io::Result<()> {
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
