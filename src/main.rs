// src/main.rs

use std::{env, fs, io::{self, Read}};
use pulldown_cmark::Parser;
mod renderer; // Declare the renderer module
mod ansi_styles; // Declare the ansi_styles module

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <markdown_file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let mut file = fs::File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let parser = Parser::new(&content);

    // Call our new rendering function
    renderer::render_markdown_to_terminal(parser)?;

    Ok(())
}
