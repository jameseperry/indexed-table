use indexed_table::Key;
use indexed_table::TableIndex;

#[derive(Clone)]
struct Row {
    id: i32,
    tag: String,
}

#[test]
fn table_index_insert_find_remove() {
    let mut idx = TableIndex::new(|r: &Row| r.id);
    let mut keys = slotmap::DenseSlotMap::<Key, ()>::new();
    let key_a = keys.insert(());
    let key_b = keys.insert(());

    let row_a = Row { id: 7, tag: "a".to_string() };
    let row_b = Row { id: 7, tag: "b".to_string() };

    idx.insert(&row_a, key_a);
    idx.insert(&row_b, key_b);

    let found = idx.find_one(&7);
    assert!(found.is_some());
    assert!(found == Some(key_a) || found == Some(key_b));

    let all = idx.find_all(&7).unwrap();
    assert_eq!(all.len(), 2);
    assert!(all.contains(&key_a));
    assert!(all.contains(&key_b));

    idx.remove(&7, key_a);
    let remaining = idx.find_all(&7).unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0], key_b);

    idx.remove(&7, key_b);
    assert!(idx.find_one(&7).is_none());
}

#[test]
fn table_index_remove_by_row_and_clear() {
    let mut idx = TableIndex::new(|r: &Row| r.tag.clone());
    let mut keys = slotmap::DenseSlotMap::<Key, ()>::new();
    let key_a = keys.insert(());
    let key_b = keys.insert(());

    let row_a = Row { id: 1, tag: "alpha".to_string() };
    let row_b = Row { id: 2, tag: "beta".to_string() };

    idx.insert(&row_a, key_a);
    idx.insert(&row_b, key_b);

    idx.remove_by_row(&row_a, key_a);
    assert!(idx.find_one(&"alpha".to_string()).is_none());
    assert_eq!(idx.find_one(&"beta".to_string()), Some(key_b));

    idx.clear();
    assert!(idx.find_one(&"beta".to_string()).is_none());
}
