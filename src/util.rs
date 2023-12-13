use std::io::{self, Write};

pub fn prompt_user(text: &str, default: bool) -> bool {
    let char_y = if default { 'Y' } else { 'y' };
    let char_n = if default { 'n' } else { 'N' };
    
    loop {
        print!("{} [{}/{}]? ", text, char_y, char_n);
        
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            "" => return default,
            _ => (),
        }
    }
}