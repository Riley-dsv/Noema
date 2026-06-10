use crate::{
    database::sqlite::SQLStore,
    error::{NoemaError, NoemaResult},
};

pub fn search_in_notes(store: &SQLStore, keyword: &str) -> NoemaResult {
    let notes = store.search_content(keyword)?;

    if notes.is_empty() {
        return Err(NoemaError::NoteNotFound(keyword.to_string()));
    }

    println!("Note containing {} : ", keyword);
    for note in notes {
        println!(
            "id: {} | title: {} | updated_at: {}",
            note.id, note.title, note.updated_at
        );
    }

    Ok(())
}
