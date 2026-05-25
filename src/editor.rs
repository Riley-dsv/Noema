use std::{env, fs, io::Write, process::Command};

use tempfile::NamedTempFile;

pub fn open_editor(initial_content: &str) -> std::io::Result<String> {
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    let mut file = NamedTempFile::new()?;
    write!(file, "{initial_content}")?;

    let path = file.path().to_owned();

    let status = Command::new(editor).arg(&path).status()?;

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "editor exited with an error",
        ));
    }

    let content = fs::read_to_string(&path)?;

    Ok(content)
}
