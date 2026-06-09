use crate::{database::sqlite::SQLStore, error::NoemaResult};
use std::io;

pub fn delete_note(store: &SQLStore, id: &str) -> NoemaResult {
    print!("Are you sure you want to delete note: {} (Y/N) > ", &id);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read stdin");

    if input.trim().to_lowercase() == "y" {
        store.delete_note(&id)?;
    }

    Ok(())
}
