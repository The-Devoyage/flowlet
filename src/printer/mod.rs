use colored::*;
use prettytable::{Cell, Row, Table};

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
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Icon::Success => Color::Green,
            Icon::Error => Color::Red,
            Icon::Failure => Color::Red,
            Icon::Warning => Color::Yellow,
            Icon::Trash => Color::BrightBlack,
            Icon::Cloud => Color::BrightBlue,
            Icon::Local => Color::Cyan,
            Icon::Info => Color::BrightBlue,
            Icon::Auth => Color::Magenta,
        }
    }
}

pub struct Printer;

impl Printer {
    pub fn success(icon: Icon, label: &str, message: &str) {
        println!(
            "{} {} {}",
            icon.symbol(),
            label.color(icon.color()).bold(),
            message
        );
    }

    pub fn error(icon: Icon, label: &str, message: &str) {
        eprintln!(
            "{} {} {}",
            icon.symbol(),
            label.color(icon.color()).bold(),
            message
        );
    }

    pub fn warning(icon: Icon, label: &str, message: &str) {
        println!(
            "{} {} {}",
            icon.symbol(),
            label.color(icon.color()).bold(),
            message
        );
    }

    pub fn info(icon: Icon, label: &str, message: &str) {
        println!(
            "{} {} {}",
            icon.symbol(),
            label.color(icon.color()).bold(),
            message
        );
    }

    pub fn field(icon: Icon, label: &str, value: &str) {
        let pad = 14;
        println!(
            "{} {}{}{}",
            icon.symbol(),
            label.bold(),
            ":".bold(),
            format!("{:>width$}", value, width = pad - label.len())
        );
    }

    pub fn table(headers: Vec<&str>, rows: Vec<Vec<String>>) {
        let mut table = Table::new();

        table.add_row(Row::new(
            headers
                .into_iter()
                .map(|h| Cell::new(h).style_spec("bFc"))
                .collect(),
        ));

        for row in rows {
            table.add_row(Row::new(row.into_iter().map(|v| Cell::new(&v)).collect()));
        }

        table.printstd();
    }
}
