mod config;
use config::Config;
mod replace;
use std::process;

fn main() {
    let matches = config::matches();
    let config = Config::new(&matches);
    
    if let Err(e) = replace::replace_text_in_files(&config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
