use crate::{error::NoemaResult, store::sqlite::notes::NoteStore};

pub fn read_note<Store: NoteStore>(store: &Store, id: &str) -> NoemaResult {
    let content = store.get_content(&id)?;
    println!("{content}");

    Ok(())
}
