mod config;
use config::Config;
mod error;
mod replace;
mod util;
use crate::error::Result;
use std::process;

fn run() -> Result<()> {
    let matches = config::matches();
    let config: Config = Config::new(&matches)?;
    replace::replace_text_in_files(&config)?;
    Ok(())
}

fn main() {
    run().unwrap_or_else(|e| {
        let stderr = std::io::stderr();
        error::default_error_handler(&e, &mut stderr.lock());
        process::exit(1);
    })
}
