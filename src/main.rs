mod config;
use config::Config;
mod replace;
mod util;
use std::process;

fn main() {
    let matches = config::matches();
    let config: Config = Config::new(&matches).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });
    
    if let Err(e) = replace::replace_text_in_files(&config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
