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
    let mut in_blockquote = false;
    let mut current_list_number: Option<u64> = None;
    let mut list_indent_level: usize = 0;

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
                        list_indent_level += 1;
                        print!("\n");
                        current_list_number = start_num;
                    }
                    Tag::Item => {
                        for _ in 0..list_indent_level -1 {
                            print!("    ");
                        }
                        if let Some(num) = &mut current_list_number {
                            print!("{}. ", num);
                            *num += 1;
                        } else {
                            print!("* ");
                        }
                    }
                    // FIX: BlockQuote handling - changed to Tag::BlockQuote(_)
                    Tag::BlockQuote(_) => {
                        in_blockquote = true;
                        print!("\n{}{}>{} ", ansi_styles::YELLOW, ansi_styles::BOLD, ansi_styles::RESET);
                    }
                    Tag::Link { dest_url, title, .. } => {
                        // For a simple implementation, we'll print the link text when Event::Text occurs.
                        // To show the URL after the text, you'd store `dest_url` in a state variable here
                        // and print it at TagEnd::Link.
                    }
                    Tag::FootnoteDefinition(label) => {
                        // Ignoring for now
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
                        list_indent_level -= 1;
                        current_list_number = None;
                        println!();
                    }
                    TagEnd::Item => {
                        println!();
                    }
                    // FIX: BlockQuote handling - changed to TagEnd::BlockQuote(_)
                    TagEnd::BlockQuote(_) => {
                        in_blockquote = false;
                        println!();
                    }
                    // FIX: Link handling - changed to TagEnd::Link
                    TagEnd::Link => {
                         print!("{}", ansi_styles::RESET); // Just ensure formatting is reset
                    }
                    _ => {}
                }
            }
            Event::Text(text) => {
                if in_code_block {
                    print!("{}{}{}", ansi_styles::YELLOW, text, ansi_styles::RESET);
                } else if in_blockquote {
                    print!("{}{}", ansi_styles::GREEN, text);
                } else {
                    print!("{}", text);
                }
            }
            Event::Code(text) => {
                print!("{}{}{}", ansi_styles::YELLOW, text, ansi_styles::RESET);
            }
            Event::SoftBreak => print!(" "),
            Event::HardBreak => println!(),
            Event::Rule => {
                println!("\n{}{}----------------------------------------{}{}",
                    ansi_styles::BLUE, ansi_styles::BOLD, ansi_styles::RESET, ansi_styles::RESET);
                println!();
            }
            Event::Html(html) => {
                // Skipping HTML for now
            }
            Event::FootnoteReference(label) => {
                // Ignoring footnote references for now.
            }
            Event::TaskListMarker(checked) => {
                if checked {
                    print!("[x] ");
                } else {
                    print!("[ ] ");
                }
            }
            Event::InlineHtml(html) => {
                // Skipping inline HTML
            }
            _ => {}
        }
        io::stdout().flush()?;
    }
    println!("{}", ansi_styles::RESET);

    Ok(())
}
