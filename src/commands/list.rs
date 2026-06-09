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
