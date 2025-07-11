use colored::*;
use prettytable::{Cell, Row, Table};
use regex;
use unicode_width::UnicodeWidthStr;

pub enum Icon {
    Trash,
    Cloud,
    Local,
    Success,
    Error,
    Warning,
    Failure,
    Info,
    Auth,
    Rocket,
    Project,
}

impl Icon {
    pub fn symbol(&self) -> &'static str {
        match self {
            Icon::Trash => "🗑️",
            Icon::Cloud => "☁️",
            Icon::Local => "💾",
            Icon::Success => "✅",
            Icon::Error => "❌",
            Icon::Warning => "⚠️",
            Icon::Failure => "💥",
            Icon::Info => "ℹ️",
            Icon::Auth => "🔐",
            Icon::Rocket => "🚀",
            Icon::Project => "📁",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Icon::Success => Color::Green,
            Icon::Rocket => Color::Green,
            Icon::Error | Icon::Failure => Color::Red,
            Icon::Warning => Color::Yellow,
            Icon::Trash => Color::BrightBlack,
            Icon::Cloud => Color::BrightBlue,
            Icon::Local => Color::Cyan,
            Icon::Info => Color::BrightBlue,
            Icon::Auth => Color::Magenta,
            Icon::Project => Color::Blue,
        }
    }

    pub fn formatted(&self) -> String {
        match self {
            Icon::Cloud | Icon::Info => format!("{}   ", self.symbol()), // 3 spaces
            Icon::Success | Icon::Trash | Icon::Auth | Icon::Rocket | Icon::Project => {
                format!("{}  ", self.symbol())
            } // 2 spaces
            Icon::Warning | Icon::Failure | Icon::Error => format!("{}  ", self.symbol()),
            Icon::Local => format!("{}  ", self.symbol()),
        }
    }
}

pub struct Printer;

impl Printer {
    pub fn success(icon: Icon, label: &str, message: &str) {
        println!(
            "{}{} {}",
            icon.formatted(),
            pad_colored(label.color(icon.color()).bold(), 12),
            message
        );
    }

    pub fn error(icon: Icon, label: &str, message: &str) {
        eprintln!(
            "{}{} {}",
            icon.formatted(),
            pad_colored(label.color(icon.color()).bold(), 12),
            message
        );
    }

    pub fn warning(icon: Icon, label: &str, message: &str) {
        println!(
            "{}{} {}",
            icon.formatted(),
            pad_colored(label.color(icon.color()).bold(), 12),
            message
        );
    }

    pub fn info(icon: Icon, label: &str, message: &str) {
        println!(
            "{}{} {}",
            icon.formatted(),
            pad_colored(label.color(icon.color()).bold(), 12),
            message
        );
    }

    pub fn field(icon: Icon, label: &str, value: &str) {
        let pad = 14;
        println!(
            "{}{}{}{}",
            icon.formatted(),
            label.bold(),
            ":".bold(),
            format!("{:>width$}", value, width = pad - label.len())
        );
    }

    pub fn table(headers: Vec<&str>, rows: Vec<Vec<String>>) {
        let mut table = Table::new();

        // Headers
        table.add_row(Row::new(
            headers
                .into_iter()
                .map(|h| Cell::new(h).style_spec("Fb"))
                .collect(),
        ));

        // Rows
        for row in rows {
            table.add_row(Row::new(
                row.iter().map(|s| Cell::new(s.as_str())).collect(),
            ));
        }

        table.printstd();
    }

    pub fn multi_line_info(label: &str, lines: Vec<&str>) {
        println!("\n{}", label.bold());
        for line in lines {
            println!("{}", line.bright_white().bold());
        }
        println!();
    }

    pub fn multi_line_info_with_icon(icon: Icon, label: &str, lines: Vec<&str>) {
        println!(
            "\n{} {}\n",
            icon.formatted(),
            label.color(icon.color()).bold()
        );
        for line in lines {
            println!("{}", line.bright_white().bold());
        }
        println!();
    }
}

/// Pads a colored string to a fixed display width using unicode-width
fn pad_colored(text: ColoredString, width: usize) -> ColoredString {
    let plain = strip_ansi_codes(&text.to_string());
    let display_width = UnicodeWidthStr::width(plain.as_str());
    let pad = width.saturating_sub(display_width);
    format!("{}{}", text, " ".repeat(pad)).normal()
}

/// Strips ANSI escape codes from a string
fn strip_ansi_codes(s: &str) -> String {
    // Crude way to remove ANSI color codes: they all start with ESC [
    // A more robust solution would use the `strip-ansi-escapes` crate
    let re = regex::Regex::new(r"\x1B\[[0-9;]*[a-zA-Z]").unwrap();
    re.replace_all(s, "").to_string()
}
