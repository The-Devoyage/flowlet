use colored::*;

pub struct Printer;

impl Printer {
    pub fn success(label: &str, message: &str) {
        println!(
            "{} {}",
            Self::tag("SUCCESS", Color::BrightGreen, Color::Black),
            Self::format(label, message)
        );
    }

    pub fn error(label: &str, message: &str) {
        eprintln!(
            "{} {}",
            Self::tag("ERROR", Color::Red, Color::White),
            Self::format(label, message)
        );
    }

    pub fn warning(label: &str, message: &str) {
        println!(
            "{} {}",
            Self::tag("WARNING", Color::BrightYellow, Color::Black),
            Self::format(label, message)
        );
    }

    pub fn info(label: &str, message: &str) {
        println!(
            "{} {}",
            Self::tag("INFO", Color::BrightBlue, Color::White),
            Self::format(label, message)
        );
    }

    fn tag(text: &str, bg: Color, fg: Color) -> ColoredString {
        format!(" {} ", text).on_color(bg).color(fg).bold()
    }

    fn format(label: &str, message: &str) -> String {
        format!("{}: {}", label.bold(), message)
    }
}
