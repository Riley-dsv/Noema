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
