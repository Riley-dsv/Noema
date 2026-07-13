use crate::{
    editor::open_editor,
    error::NoemaResult,
    store::sqlite::{note_tags::NoteTagsStore, notes::NoteStore, tags::TagsStore},
};

pub fn update_note<Store: NoteStore>(store: &Store, id: &str, title: Option<&str>) -> NoemaResult {
    if let Some(title) = title {
        store.update_title(&id, &title)?;
    } else {
        let old_content = store.get_content(&id)?;
        let new_content = open_editor(&old_content)?;
        store.update_content(&id, &new_content)?;
    }

    Ok(())
}

pub fn attach_tag_to_note<Store: TagsStore + NoteTagsStore>(
    store: &Store,
    note_id: &str,
    tag_name: &str,
) -> NoemaResult {
    if !store.tag_exists(tag_name)? {
        println!("Tag: {} Does not exists", tag_name);
        return Ok(());
    }

    let tag_id = store.get_id_from_tag_name(tag_name)?;
    store.update_note_tags(note_id, &tag_id)?;

    Ok(())
}
