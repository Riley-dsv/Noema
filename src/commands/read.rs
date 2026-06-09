use crate::{database::sqlite::SQLStore, error::NoemaResult};

pub fn read_note(store: &SQLStore, id: &str) -> NoemaResult {
    let content = store.get_content(&id)?;
    println!("{content}");

    Ok(())
}
