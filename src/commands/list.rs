use crate::{database::sqlite::SQLStore, error::NoemaResult};

pub fn list_notes(store: &SQLStore) -> NoemaResult {
    let notes = store.list_notes()?;
    for note in notes {
        println!(
            "id: {} | title: {} | updated_at: {}",
            note.id, note.title, note.updated_at
        );
    }

    Ok(())
}

pub fn list_tags(store: &SQLStore) -> NoemaResult {
    let tags = store.list_tags()?;
    for tag in tags {
        println!(
            "Tag name: {}\nAttached to {} note(s)",
            tag.name, tag.total_attached
        );
    }

    Ok(())
}
