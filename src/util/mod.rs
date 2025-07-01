use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;
use tempfile::NamedTempFile;

pub type FlowletResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn launch_editor(initial: &str) -> std::io::Result<String> {
    // Create a temp file and write the initial content
    let mut file = NamedTempFile::new()?;
    write!(file, "{}", initial)?;

    // Use $EDITOR or fallback
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| {
        if which::which("vim").is_ok() {
            "vim".to_string()
        } else if which::which("vi").is_ok() {
            "vi".to_string()
        } else {
            "nano".to_string()
        }
    });

    // Open the editor
    let status = Command::new(editor).arg(file.path()).status()?;

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Editor exited with error",
        ));
    }

    // Read the edited contents back
    let mut content = String::new();
    let mut f = File::open(file.path())?;
    f.read_to_string(&mut content)?;

    Ok(content)
}

pub fn clean_command(raw: &str) -> String {
    raw.lines()
        .map(str::trim_end) // remove trailing whitespace on each line
        .collect::<Vec<_>>()
        .join(" ") // join lines with space
        .replace('\\', "") // remove literal backslashes
        .replace("  ", " ") // collapse double spaces
        .trim() // final trim
        .to_string()
}

pub fn extract_json_path<'a>(
    value: &'a serde_json::Value,
    path: &str,
) -> Option<&'a serde_json::Value> {
    let mut current = value;
    for key in path.split('.') {
        current = current.get(key)?;
    }
    Some(current)
}
