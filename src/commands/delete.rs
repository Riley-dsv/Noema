use std::io::{self, Write};

use crate::{database::sqlite::SQLStore, error::NoemaResult};

pub fn delete_note(store: &SQLStore, id: &str) -> NoemaResult {
    print!("Are you sure you want to delete note: {} (Y/N) > ", id);
    io::stdout().flush()?;

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read stdin");

    if input.trim().eq_ignore_ascii_case("y") {
        store.delete_note(id)?;
    }

    Ok(())
}
