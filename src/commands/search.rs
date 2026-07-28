use crate::{
    error::NoemaResult,
    search::{note::search_for_notes, search_options::SearchOptions},
    store::sqlite::{lookup::LookupStore, tags::TagsStore},
};

pub fn search_notes<Store: LookupStore + TagsStore>(
    store: &Store,
    options: SearchOptions<'_>,
) -> NoemaResult {
    for note in search_for_notes(store, options)? {
        println!("{} | {} | {}", note.id, note.title, note.updated_at);
    }

    Ok(())
}

pub fn search_in_notes<Store: LookupStore + TagsStore>(
    store: &Store,
    keyword: Option<&str>,
    tag: Option<&str>,
) -> NoemaResult {
    search_notes(
        store,
        SearchOptions {
            keyword,
            tags: tag.map(|tag| vec![tag]),
            match_any: false,
            search_title: false,
            search_content: false,
            limit: None,
        },
    )
}
