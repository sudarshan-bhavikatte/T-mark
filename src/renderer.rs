// src/renderer.rs

use std::io::{self, Write};
use pulldown_cmark::{Event, Tag, TagEnd, Parser};
use crate::ansi_styles;

/// Renders markdown events to the terminal using ANSI escape codes.
///
/// # Arguments
/// * `parser` - A `pulldown_cmark::Parser` instance yielding events.
///
/// # Returns
/// An `io::Result<()>` indicating success or failure.
pub fn render_markdown_to_terminal(parser: Parser) -> io::Result<()> {
    let mut in_strong = false;
    let mut in_emphasis = false;
    let mut in_heading = false;
    let mut in_code_block = false;
    let mut current_list_number: Option<u64> = None;
    let mut list_indent_level: usize = 0; // NEW: Track list nesting depth

    for event in parser {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Heading { level, .. } => {
                        in_heading = true;
                        match level {
                            pulldown_cmark::HeadingLevel::H1 => print!("\n{}{}{} ", ansi_styles::BOLD, ansi_styles::CYAN, ansi_styles::UNDERLINE),
                            pulldown_cmark::HeadingLevel::H2 => print!("\n{}{}{}## ", ansi_styles::BOLD, ansi_styles::MAGENTA, ansi_styles::UNDERLINE),
                            pulldown_cmark::HeadingLevel::H3 => print!("\n{}{}{}### ", ansi_styles::BOLD, ansi_styles::BLUE, ansi_styles::UNDERLINE),
                            _ => print!("\n{}{}{}# ", ansi_styles::BOLD, ansi_styles::WHITE, ansi_styles::UNDERLINE),
                        }
                    }
                    Tag::Paragraph => print!(""),
                    Tag::Strong => {
                        in_strong = true;
                        print!("{}", ansi_styles::BOLD);
                    }
                    Tag::Emphasis => {
                        in_emphasis = true;
                        print!("{}", ansi_styles::ITALIC);
                    }
                    Tag::CodeBlock(kind) => {
                        in_code_block = true;
                        print!("\n{}{}{}```{}\n", ansi_styles::BLUE, ansi_styles::BOLD, ansi_styles::RESET, ansi_styles::CYAN);
                    }
                    Tag::List(start_num) => {
                        list_indent_level += 1; // Increase indent for nested lists
                        print!("\n"); // Newline before a list
                        current_list_number = start_num;
                    }
                    Tag::Item => {
                        // Print current indentation
                        for _ in 0..list_indent_level -1 { // Adjust for item prefix space
                            print!("    "); // 4 spaces per indent level
                        }
                        if let Some(num) = &mut current_list_number {
                            print!("{}. ", num);
                            *num += 1;
                        } else {
                            print!("* ");
                        }
                    }
                    _ => {}
                }
            }
            Event::End(tag_end) => {
                match tag_end {
                    TagEnd::Heading(_) => {
                        in_heading = false;
                        println!("{}", ansi_styles::RESET);
                        println!();
                    }
                    TagEnd::Paragraph => {
                        println!();
                    }
                    TagEnd::Strong => {
                        in_strong = false;
                        print!("{}", ansi_styles::RESET);
                        if in_emphasis { print!("{}", ansi_styles::ITALIC); }
                    }
                    TagEnd::Emphasis => {
                        in_emphasis = false;
                        print!("{}", ansi_styles::RESET);
                        if in_strong { print!("{}", ansi_styles::BOLD); }
                    }
                    TagEnd::CodeBlock => {
                        in_code_block = false;
                        print!("{}{}```{}\n", ansi_styles::BLUE, ansi_styles::BOLD, ansi_styles::RESET);
                        println!();
                    }
                    TagEnd::List(_) => {
                        list_indent_level -= 1; // Decrease indent when list ends
                        current_list_number = None; // This needs to be more robust for nested ordered lists
                                                    // For deeply nested ordered lists, you'd need a stack of numbers
                        println!(); // Add a newline after the list
                    }
                    TagEnd::Item => { // NEW: Add a newline after each list item ends
                        println!();
                    }
                    _ => {}
                }
            }
            Event::Text(text) => {
                if in_code_block {
                    print!("{}{}{}", ansi_styles::YELLOW, text, ansi_styles::RESET);
                } else {
                    print!("{}", text);
                }
            }
            Event::Code(text) => {
                print!("{}{}{}", ansi_styles::YELLOW, text, ansi_styles::RESET);
            }
            Event::SoftBreak => print!(" "),
            Event::HardBreak => println!(),
            _ => {}
        }
        io::stdout().flush()?;
    }
    println!("{}", ansi_styles::RESET);

    Ok(())
}
