use crate::{
    editor::open_editor,
    error::NoemaResult,
    store::sqlite::{notes::NoteStore, tags::TagsStore},
};

pub fn create_note<Store: NoteStore>(
    store: &Store,
    title: &str,
    content: Option<&str>,
) -> NoemaResult {
    let editor_content = open_editor(content.unwrap_or_default())?;
    let id = store.insert_note(title, &editor_content)?;
    println!("Note {} created with ID: {}", title, id);
    Ok(())
}
pub fn create_tag<Store: TagsStore>(store: &Store, tag_name: &str) -> NoemaResult {
    store.insert_tag(tag_name)?;
    println!("Tag {} created.", tag_name);
    Ok(())
}
