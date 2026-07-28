use noema::store::sqlite::{
    SQLStore, lookup::LookupStore, note_tags::NoteTagsStore, notes::NoteStore, tags::TagsStore,
};

fn store() -> SQLStore {
    let mut store = SQLStore::open_in_memory().expect("in-memory store should open");
    store.init().expect("store should initialize");
    store
}

fn note_and_tag(store: &SQLStore, title: &str, content: &str, tag: &str) -> (String, i32) {
    let note_id = store.insert_note(title, content).unwrap();
    store.insert_tag(tag).unwrap();
    let tag_id = store.get_id_from_tag_name(tag).unwrap();
    (note_id, tag_id)
}

#[test]
fn attaches_and_detaches_tags_idempotently() {
    let store = store();
    let (note_id, tag_id) = note_and_tag(&store, "Traits", "Store traits", "rust");

    store.update_note_tags(&note_id, &tag_id).unwrap();
    store.update_note_tags(&note_id, &tag_id).unwrap();

    assert_eq!(store.filter_tags_by_note(&note_id).unwrap(), ["rust"]);
    assert_eq!(store.filter_notes_by_tag(&tag_id).unwrap().len(), 1);
    assert_eq!(store.delete_tag_from_note(&note_id, &tag_id).unwrap(), 1);
    assert_eq!(store.delete_tag_from_note(&note_id, &tag_id).unwrap(), 0);
}

#[test]
fn reports_attachment_counts_and_filters_both_sides_of_the_relationship() {
    let store = store();
    let first = store.insert_note("First", "").unwrap();
    let second = store.insert_note("Second", "").unwrap();
    store.insert_tag("rust").unwrap();
    store.insert_tag("zettelkasten").unwrap();
    let rust = store.get_id_from_tag_name("rust").unwrap();
    let zettelkasten = store.get_id_from_tag_name("zettelkasten").unwrap();

    store.update_note_tags(&first, &rust).unwrap();
    store.update_note_tags(&second, &rust).unwrap();
    store.update_note_tags(&first, &zettelkasten).unwrap();

    let rust_notes = store.filter_notes_by_tag(&rust).unwrap();
    assert_eq!(rust_notes.len(), 2);
    assert!(rust_notes.iter().any(|note| note.id == first));
    assert!(rust_notes.iter().any(|note| note.id == second));

    let mut first_tags = store.filter_tags_by_note(&first).unwrap();
    first_tags.sort();
    assert_eq!(first_tags, ["rust", "zettelkasten"]);

    let tags = store.list_tags().unwrap();
    assert_eq!(
        tags.iter()
            .find(|tag| tag.name == "rust")
            .unwrap()
            .total_attached,
        2
    );
}

#[test]
fn searches_titles_and_content_independently_with_partial_case_insensitive_matches() {
    let store = store();
    let title_match = store.insert_note("Rust ownership", "Unrelated").unwrap();
    let content_match = store
        .insert_note("Zettelkasten", "Notes about RUST traits")
        .unwrap();
    store.insert_note("Other", "Nothing to see").unwrap();

    let title_results = store.search_title("rust").unwrap();
    assert_eq!(title_results.len(), 1);
    assert_eq!(title_results[0].id, title_match);

    let content_results = store.search_content("rust").unwrap();
    assert_eq!(content_results.len(), 1);
    assert_eq!(content_results[0].id, content_match);
    assert!(store.search_content("absent").unwrap().is_empty());
}

#[test]
fn deleting_a_note_or_tag_cascades_to_attachments() {
    let store = store();
    let (note_id, tag_id) = note_and_tag(&store, "Linked", "", "linked");
    store.update_note_tags(&note_id, &tag_id).unwrap();

    store.delete_note(&note_id).unwrap();
    assert!(store.filter_notes_by_tag(&tag_id).unwrap().is_empty());

    let second_note = store.insert_note("Second", "").unwrap();
    store.update_note_tags(&second_note, &tag_id).unwrap();
    store.delete_tag(&tag_id).unwrap();
    assert!(store.filter_tags_by_note(&second_note).unwrap().is_empty());
}

#[test]
fn foreign_keys_reject_attachments_to_missing_records() {
    let store = store();
    let note_id = store.insert_note("Existing", "").unwrap();

    assert!(store.update_note_tags(&note_id, &999_999).is_err());
}
