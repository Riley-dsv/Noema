use crate::{database::sqlite::SQLStore, error::NoemaResult};

pub fn note_info(store: &SQLStore, id: &str) -> NoemaResult {
    let note = store.get_note(&id)?;
    let content_size = note.content.len();
    let tags: String = store.filter_tags_by_note(&id)?.join(", ");
    println!(
        "ID: {}\nTitle: {}\nCreated At: {}\nLast Updated: {}\nSize: {}Bytes\nTags: {}",
        &id, note.title, note.created_at, note.updated_at, content_size, tags
    );

    Ok(())
}
