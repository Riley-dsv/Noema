use noema::store::sqlite::{SQLStore, notes::NoteStore};
use rusqlite::Error;

fn store() -> SQLStore {
    let mut store = SQLStore::open_in_memory().expect("in-memory store should open");
    store.init().expect("store should initialize");
    store
}

#[test]
fn creates_and_reads_a_note() {
    let store = store();

    let id = store
        .insert_note("Test", "Hello")
        .expect("note should be inserted");
    let note = store.get_note(&id).expect("note should be readable");

    assert_eq!(note.id, id);
    assert_eq!(note.title, "Test");
    assert_eq!(note.content, "Hello");
    assert!(!note.created_at.is_empty());
    assert!(!note.updated_at.is_empty());
}

#[test]
fn updates_a_notes_title_and_content_without_changing_its_identity() {
    let store = store();
    let id = store
        .insert_note("Old title", "Old content")
        .expect("note should be inserted");

    store
        .update_title(&id, "New title")
        .expect("title should update");
    store
        .update_content(&id, "New content")
        .expect("content should update");

    let note = store.get_note(&id).expect("updated note should exist");
    assert_eq!(note.id, id);
    assert_eq!(note.title, "New title");
    assert_eq!(note.content, "New content");
}

#[test]
fn lists_all_notes_and_gets_content_directly() {
    let store = store();
    let first_id = store
        .insert_note("First", "First body")
        .expect("first note should be inserted");
    let second_id = store
        .insert_note("Second", "Second body")
        .expect("second note should be inserted");

    let notes = store.list_notes().expect("notes should be listed");
    let mut ids: Vec<_> = notes.into_iter().map(|note| note.id).collect();
    ids.sort();

    let mut expected = vec![first_id.clone(), second_id];
    expected.sort();
    assert_eq!(ids, expected);
    assert_eq!(
        store
            .get_content(&first_id)
            .expect("content should be readable"),
        "First body"
    );
}

#[test]
fn deleting_a_note_reports_whether_a_row_was_removed() {
    let store = store();
    let id = store
        .insert_note("Disposable", "Body")
        .expect("note should be inserted");

    assert_eq!(store.delete_note(&id).expect("note should delete"), 1);
    assert_eq!(
        store
            .delete_note(&id)
            .expect("deleting a missing note should succeed"),
        0
    );
    assert!(matches!(
        store.get_note(&id),
        Err(Error::QueryReturnedNoRows)
    ));
}

#[test]
fn generated_note_ids_are_short_unique_hex_strings() {
    let store = store();

    let first = store.insert_note("First", "").unwrap();
    let second = store.insert_note("Second", "").unwrap();

    assert_eq!(first.len(), 8);
    assert!(first.chars().all(|character| character.is_ascii_hexdigit()));
    assert_ne!(first, second);
}
