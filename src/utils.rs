use colored::Colorize;
use tabled::settings::{Color, Format, Settings, Style, formatting::Justification, object::Rows};

pub fn format_line_section_highlight(input: &str, start_index: usize, end_index: usize) -> String {
    format!(
        "\n\t{}\n\t{}{}",
        input.trim().dimmed(),
        " ".repeat(start_index),
        "^".repeat(end_index - start_index + 1).red().bold()
    )
}

pub fn format_message(module: &str, message: &str) -> String {
    format!("{}: {}", module, message).bold().to_string()
}

pub fn format_table(table: &mut tabled::Table) -> &mut tabled::Table {
    let settings = Settings::default().with(Style::rounded());
    table.modify(Rows::first(), Color::BOLD);
    table.with(settings)
}
