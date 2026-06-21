use std::io::{self, Error, Write};

use crate::{database::sqlite::SQLStore, error::NoemaResult};

pub fn delete_note(store: &SQLStore, id: &str) -> NoemaResult {
    let confirmation = confirm_action(format!("Are you sure you want to delete the note: {}", id))?;

    if confirmation {
        let deleted = store.delete_note(id)?;
        println!("Deleted {} Note(s)", deleted);
    }

    Ok(())
}

pub fn detach_tag_from_note(store: &SQLStore, note_id: &str, tag_name: &str) -> NoemaResult {
    if !store.tag_exists(tag_name).unwrap() {
        println!("Tag: {} Does not exists", tag_name);
        return Ok(());
    }

    let tag_id = store.get_id_from_tag_name(tag_name)?;

    let confirmation = confirm_action(format!(
        "Are you sure want to detach the tag {} from the note: {}",
        tag_name, note_id
    ))?;

    if confirmation {
        let deleted = store.delete_tag_from_note(note_id, &tag_id)?;
        println!("Detached {} tag(s)", deleted);
    }

    Ok(())
}

fn confirm_action(message: String) -> Result<bool, Error> {
    print!("{message} (Y/N) > ");
    io::stdout().flush()?;

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read stdin");

    let confirmation = input.trim().eq_ignore_ascii_case("y");

    Ok(confirmation)
}
