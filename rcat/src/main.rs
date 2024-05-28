use colored::Colorize;
use std::{error::Error, fs};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about, author)]
struct Args {
    #[arg(index = 1)]
    file_path: String,
}

fn read_file(path: String) -> Result<String, Box<dyn Error>> {
    let file_content = fs::read_to_string(path)?;

    Ok(file_content)
}

fn main() {
    let args = Args::parse();

    match read_file(args.file_path) {
        Ok(content) => println!("{content}"),
        Err(e) => println!("{}: {}", "Failed to read the file!".red().bold(), e),
    }
}
