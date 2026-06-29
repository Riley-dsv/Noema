use std::collections::HashSet;

use crate::{
    database::sqlite::{NoteSummary, SQLStore},
    error::{NoemaError, NoemaResult},
};

fn search_by_tag(store: &SQLStore, tag: &str) -> Result<Vec<NoteSummary>, NoemaError> {
    let tag_exists = store.tag_exists(tag)?;

    if !tag_exists {
        println!("No tags with the name: {} found.", tag);
        return Ok(vec![]);
    }

    let tag_id = store.get_id_from_tag_name(tag)?;
    let notes = store.filter_notes_by_tag(&tag_id)?;

    if notes.is_empty() {
        println!("No note with tag : {} found.", tag);
        return Ok(vec![]);
    }

    Ok(notes)
}

fn search_by_keyword(store: &SQLStore, keyword: &str) -> Result<Vec<NoteSummary>, NoemaError> {
    let notes = store.search_content(keyword)?;

    if notes.is_empty() {
        println!("No note with keyword: {} found", keyword);
        return Ok(vec![]);
    }

    Ok(notes)
}

fn search_by_tag_and_keyword(
    store: &SQLStore,
    keyword: &str,
    tag: &str,
) -> Result<Vec<NoteSummary>, NoemaError> {
    let notes_found_by_keyword = search_by_keyword(store, keyword)?;
    let notes_found_by_tag = search_by_tag(store, tag)?;

    let note_ids: HashSet<_> = notes_found_by_keyword
        .iter()
        .map(|note| note.id.as_str())
        .collect();

    let intersection: Vec<_> = notes_found_by_tag
        .into_iter()
        .filter(|note| note_ids.contains(note.id.as_str()))
        .collect();

    if intersection.is_empty() {
        println!(
            "No notes found sharing the tag {} AND the keyword {}.\nYou should search only by tag or by keyword.",
            tag, keyword
        );
        return Ok(vec![]);
    }

    Ok(intersection)
}

pub fn search_in_notes(store: &SQLStore, keyword: Option<&str>, tag: Option<&str>) -> NoemaResult {
    let notes: Vec<NoteSummary> = match (tag, keyword) {
        (Some(tag), Some(keyword)) => search_by_tag_and_keyword(store, keyword, tag)?,
        (Some(tag), None) => search_by_tag(store, tag)?,
        (None, Some(keyword)) => search_by_keyword(store, keyword)?,
        (None, None) => {
            return Err(NoemaError::SearchFailed(
                "You provided no tags and no keyword, how did you do this ?".to_string(),
            ));
        }
    };

    if !notes.is_empty() {
        for note in notes {
            println!("{} | {} | {}", note.id, note.title, note.updated_at);
        }
    }

    Ok(())
}
