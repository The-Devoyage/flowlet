use colored::*;
use prettytable::{Cell, Row, Table};

pub struct Printer;

impl Printer {
    pub fn success(label: &str, message: &str) {
        println!("{} {}", label.green().bold(), message);
    }

    pub fn error(label: &str, message: &str) {
        eprintln!("{} {}", label.red().bold(), message);
    }

    pub fn warning(label: &str, message: &str) {
        println!("{} {}", label.yellow().bold(), message);
    }

    pub fn info(label: &str, message: &str) {
        println!("{} {}", label.blue().bold(), message);
    }

    /// Prints a key-value pair aligned nicely
    pub fn field(label: &str, value: &str) {
        let pad = 14; // width for label column
        println!(
            "{}{}{}",
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
