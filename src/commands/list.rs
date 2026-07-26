use crate::{
    error::NoemaResult,
    store::sqlite::{notes::NoteStore, tags::TagsStore},
};

pub fn list_notes<Store: NoteStore>(store: &Store) -> NoemaResult {
    let notes = store.list_notes()?;
    for note in notes {
        println!(
            "id: {} | title: {} | updated_at: {}",
            note.id, note.title, note.updated_at
        );
    }

    Ok(())
}

pub fn list_tags<Store: TagsStore>(store: &Store) -> NoemaResult {
    let tags = store.list_tags()?;
    for tag in tags {
        println!(
            "Tag name: {}\nAttached to {} note(s)",
            tag.name, tag.total_attached
        );
    }

    Ok(())
}
