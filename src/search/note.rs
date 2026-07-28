use std::collections::HashSet;

use crate::{
    error::NoemaError,
    search::search_options::SearchOptions,
    store::sqlite::{lookup::LookupStore, note_tags::NoteSummary, tags::TagsStore},
};

pub fn search_by_tag<Store: TagsStore + LookupStore>(
    store: &Store,
    tag: &str,
) -> Result<Vec<NoteSummary>, NoemaError> {
    let tag_exists = store.tag_exists(tag)?;

    if !tag_exists {
        return Ok(vec![]);
    }

    let tag_id = store.get_id_from_tag_name(tag)?;
    let notes = store.filter_notes_by_tag(&tag_id)?;

    if notes.is_empty() {
        return Ok(vec![]);
    }

    Ok(notes)
}

pub fn search_for_content<Store: LookupStore>(
    store: &Store,
    keyword: &str,
) -> Result<Vec<NoteSummary>, NoemaError> {
    let notes = store.search_content(keyword)?;

    if notes.is_empty() {
        return Ok(vec![]);
    }

    Ok(notes)
}

pub fn search_for_title<Store: LookupStore>(
    store: &Store,
    keyword: &str,
) -> Result<Vec<NoteSummary>, NoemaError> {
    let notes = store.search_title(keyword)?;

    if notes.is_empty() {
        return Ok(vec![]);
    }

    Ok(notes)
}

fn intersection_of(set_a: Vec<NoteSummary>, set_b: Vec<NoteSummary>) -> Vec<NoteSummary> {
    let note_ids: HashSet<_> = set_b.iter().map(|note| note.id.as_str()).collect();

    set_a
        .into_iter()
        .filter(|note| note_ids.contains(note.id.as_str()))
        .collect()
}

fn union_of(set_a: Vec<NoteSummary>, set_b: Vec<NoteSummary>) -> Vec<NoteSummary> {
    let mut seen = HashSet::new();

    set_a
        .into_iter()
        .chain(set_b)
        .filter(|note| seen.insert(note.id.clone()))
        .collect()
}

fn search_for_tags<Store: LookupStore + TagsStore>(
    store: &Store,
    tags: &[&str],
    match_any: bool,
) -> Result<Vec<NoteSummary>, NoemaError> {
    let mut tags = tags.iter();
    let Some(first_tag) = tags.next() else {
        return Ok(Vec::new());
    };

    let mut results = search_by_tag(store, first_tag)?;

    for tag in tags {
        let matches = search_by_tag(store, tag)?;
        results = if match_any {
            union_of(results, matches)
        } else {
            intersection_of(results, matches)
        };
    }

    Ok(results)
}

pub fn search_for_notes<Store: LookupStore + TagsStore>(
    store: &Store,
    search_options: SearchOptions,
) -> Result<Vec<NoteSummary>, NoemaError> {
    let has_keyword = search_options.keyword.is_some();
    let has_tags = search_options.tags.is_some();

    let keyword_results = match search_options.keyword {
        Some(keyword) => match (search_options.search_title, search_options.search_content) {
            (true, false) => search_for_title(store, keyword)?,
            (false, true) => search_for_content(store, keyword)?,
            _ => union_of(
                search_for_title(store, keyword)?,
                search_for_content(store, keyword)?,
            ),
        },
        None => Vec::new(),
    };

    let tag_results = match search_options.tags {
        Some(tags) => search_for_tags(store, &tags, search_options.match_any)?,
        None => Vec::new(),
    };

    let mut results = match (has_keyword, has_tags) {
        (true, true) => intersection_of(keyword_results, tag_results),
        (true, false) => keyword_results,
        (false, true) => tag_results,
        (false, false) => Vec::new(),
    };

    if let Some(limit) = search_options.limit {
        results.truncate(limit);
    }

    Ok(results)
}
