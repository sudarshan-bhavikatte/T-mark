mod ansi_styles;
mod renderer;

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: mdview <markdown_file>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Failed to read file");

    renderer::render_markdown(&contents);
}
