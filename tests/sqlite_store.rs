use noema::database::sqlite::SQLStore;

#[test]
fn should_create_and_read_note() {
    let store = SQLStore::open_in_memory().unwrap();

    store.init().unwrap();

    let id = store.insert_note("Test", "Hello").unwrap();
    let note = store.get_note(&id).unwrap();

    assert_eq!(note.title, "Test");
    assert_eq!(note.content, "Hello");
}

#[test]
fn should_delete_note() {
    let store = SQLStore::open_in_memory().unwrap();

    store.init().unwrap();

    let id = store.insert_note("Test", "Hello").unwrap();
    let deleted = store.delete_note(&id).unwrap();

    assert!(deleted > 0);
}

#[test]
fn should_update_note() {
    let store = SQLStore::open_in_memory().unwrap();

    store.init().unwrap();

    let id = store.insert_note("Test", "Hello").unwrap();
    let old_note = store.get_note(&id).unwrap();

    store.update_title(&id, "New Test").unwrap();
    let new_note = store.get_note(&id).unwrap();

    assert_ne!(old_note.title, new_note.title);
}

#[test]
fn should_create_tag() {
    let store = SQLStore::open_in_memory().unwrap();

    store.init().unwrap();

    let _ = store.insert_tag("Test");

    assert!(store.tag_exists("Test").unwrap());
}
