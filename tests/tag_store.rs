use noema::store::sqlite::{SQLStore, tags::TagsStore};
use rusqlite::Error;

fn store() -> SQLStore {
    let mut store = SQLStore::open_in_memory().expect("in-memory store should open");
    store.init().expect("store should initialize");
    store
}

#[test]
fn creates_finds_and_lists_tags_alphabetically() {
    let store = store();
    store.insert_tag("rust").expect("tag should be inserted");
    store
        .insert_tag("architecture")
        .expect("tag should be inserted");

    assert!(store.tag_exists("rust").unwrap());
    assert!(!store.tag_exists("missing").unwrap());

    let tags = store.list_tags().expect("tags should be listed");
    let names: Vec<_> = tags.iter().map(|tag| tag.name.as_str()).collect();
    assert_eq!(names, ["architecture", "rust"]);
    assert!(tags.iter().all(|tag| tag.total_attached == 0));
}

#[test]
fn tag_names_are_unique() {
    let store = store();
    store.insert_tag("rust").expect("tag should be inserted");

    assert!(matches!(
        store.insert_tag("rust"),
        Err(Error::SqliteFailure(_, _))
    ));
}

#[test]
fn gets_a_tag_id_and_deletes_the_tag() {
    let store = store();
    store
        .insert_tag("temporary")
        .expect("tag should be inserted");
    let id = store
        .get_id_from_tag_name("temporary")
        .expect("tag id should exist");

    assert_eq!(store.delete_tag(&id).expect("tag should delete"), 1);
    assert!(!store.tag_exists("temporary").unwrap());
    assert_eq!(store.delete_tag(&id).unwrap(), 0);
}
