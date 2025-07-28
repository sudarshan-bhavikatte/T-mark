use crate::ansi_styles::*;

pub fn render_markdown(input: &str) {
    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() {
        if line.trim_start().starts_with("|") {
            // Table block
            let mut table_lines = vec![line];
            while let Some(next) = lines.peek() {
                if next.trim_start().starts_with("|") {
                    table_lines.push(lines.next().unwrap());
                } else {
                    break;
                }
            }
            render_table(&table_lines);
        } else if line.starts_with("# ") {
            println!("{}{}{}", BOLD, &line[2..], RESET);
        } else if line.starts_with("## ") {
            println!("{}{}{}", UNDERLINE, &line[3..], RESET);
        } else if line.starts_with("```") {
            println!("{}[Code block rendering not implemented]{}", ITALIC, RESET);
        } else {
            println!("{}", line);
        }
    }
}

fn render_table(lines: &[&str]) {
    let rows: Vec<Vec<String>> = lines
        .iter()
        .map(|line| {
            line.trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_string())
                .collect()
        })
        .collect();

    if rows.len() < 2 {
        println!("Invalid table format.");
        return;
    }

    let col_count = rows[0].len();
    let mut col_widths = vec![0; col_count];

    // Calculate max column width
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            col_widths[i] = col_widths[i].max(cell.len());
        }
    }

    for (i, row) in rows.iter().enumerate() {
        let formatted: Vec<String> = row.iter().enumerate()
            .map(|(j, cell)| format!("{:<width$}", cell, width = col_widths[j]))
            .collect();

        if i == 0 {
            // Header row
            println!("{}{}{}", BOLD, formatted.join(" | "), RESET);
        } else if i == 1 {
            // Separator row
            let sep: Vec<String> = col_widths.iter()
                .map(|w| "-".repeat(*w))
                .collect();
            println!("{}", sep.join("-+-"));
        } else {
            println!("{}", formatted.join(" | "));
        }
    }
}
