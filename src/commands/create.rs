use crate::{database::sqlite::SQLStore, editor::open_editor, error::NoemaResult};

pub fn create_note(store: &SQLStore, title: &str, content: Option<&str>) -> NoemaResult {
    let editor_content = open_editor(content.unwrap_or_default())?;
    let id = store.insert_note(title, &editor_content)?;
    println!("Note {} created with ID: {}", title, id);
    Ok(())
}
