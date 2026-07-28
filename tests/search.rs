use noema::{
    search::{note::search_for_notes, search_options::SearchOptions},
    store::sqlite::{SQLStore, note_tags::NoteTagsStore, notes::NoteStore, tags::TagsStore},
};

fn store() -> SQLStore {
    let mut store = SQLStore::open_in_memory().expect("in-memory store should open");
    store.init().expect("store should initialize");
    store
}

fn options<'a>(keyword: Option<&'a str>, tags: Option<Vec<&'a str>>) -> SearchOptions<'a> {
    SearchOptions {
        keyword,
        tags,
        match_any: false,
        search_title: false,
        search_content: false,
        limit: None,
    }
}

fn attach(store: &SQLStore, note_id: &str, tag: &str) {
    if !store.tag_exists(tag).unwrap() {
        store.insert_tag(tag).unwrap();
    }
    let tag_id = store.get_id_from_tag_name(tag).unwrap();
    store.update_note_tags(note_id, &tag_id).unwrap();
}

#[test]
fn multiple_tags_require_all_tags_by_default() {
    let store = store();
    let both = store.insert_note("Both", "").unwrap();
    let only_rust = store.insert_note("Rust", "").unwrap();
    attach(&store, &both, "rust");
    attach(&store, &both, "zettelkasten");
    attach(&store, &only_rust, "rust");

    let results =
        search_for_notes(&store, options(None, Some(vec!["rust", "zettelkasten"]))).unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, both);
}

#[test]
fn match_any_returns_notes_with_at_least_one_tag_without_duplicates() {
    let store = store();
    let both = store.insert_note("Both", "").unwrap();
    let only_rust = store.insert_note("Rust", "").unwrap();
    attach(&store, &both, "rust");
    attach(&store, &both, "zettelkasten");
    attach(&store, &only_rust, "rust");

    let mut search_options = options(None, Some(vec!["rust", "zettelkasten"]));
    search_options.match_any = true;
    let results = search_for_notes(&store, search_options).unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results.iter().filter(|note| note.id == both).count(), 1);
    assert!(results.iter().any(|note| note.id == only_rust));
}

#[test]
fn keyword_search_defaults_to_title_or_content() {
    let store = store();
    let title_match = store.insert_note("Rust title", "Unrelated").unwrap();
    let content_match = store.insert_note("Unrelated", "Rust content").unwrap();

    let results = search_for_notes(&store, options(Some("Rust"), None)).unwrap();

    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|note| note.id == title_match));
    assert!(results.iter().any(|note| note.id == content_match));
}

#[test]
fn title_and_content_flags_restrict_the_keyword_search() {
    let store = store();
    let title_match = store.insert_note("Rust title", "Unrelated").unwrap();
    store.insert_note("Unrelated", "Rust content").unwrap();

    let mut search_options = options(Some("Rust"), None);
    search_options.search_title = true;
    let results = search_for_notes(&store, search_options).unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, title_match);
}

#[test]
fn keyword_and_tags_are_combined_with_and_then_limited() {
    let store = store();
    let matching = store.insert_note("Rust one", "").unwrap();
    let also_matching = store.insert_note("Rust two", "").unwrap();
    let wrong_tag = store.insert_note("Rust three", "").unwrap();
    attach(&store, &matching, "zettelkasten");
    attach(&store, &also_matching, "zettelkasten");
    attach(&store, &wrong_tag, "other");

    let mut search_options = options(Some("Rust"), Some(vec!["zettelkasten"]));
    search_options.limit = Some(1);
    let results = search_for_notes(&store, search_options).unwrap();

    assert_eq!(results.len(), 1);
    assert!(results[0].id == matching || results[0].id == also_matching);
}
