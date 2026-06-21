use crate::{database::sqlite::SQLStore, editor::open_editor, error::NoemaResult};

pub fn update_note(store: &SQLStore, id: &str, title: Option<&str>) -> NoemaResult {
    if let Some(title) = title {
        store.update_title(&id, &title)?;
    } else {
        let old_content = store.get_content(&id)?;
        let new_content = open_editor(&old_content)?;
        store.update_content(&id, &new_content)?;
    }

    Ok(())
}

pub fn attach_tag_to_note(store: &SQLStore, note_id: &str, tag_name: &str) -> NoemaResult {
    if !store.tag_exists(tag_name).unwrap() {
        println!("Tag: {} Does not exists", tag_name);
        return Ok(());
    }

    let tag_id = store.get_id_from_tag_name(tag_name)?;
    store.update_note_tags(note_id, &tag_id)?;

    Ok(())
}
