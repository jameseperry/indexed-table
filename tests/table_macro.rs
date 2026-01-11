#[derive(Clone)]
struct Row {
    foo: i32,
    bar: String,
}

#[test]
fn table_macro_updates_indices_on_insert_remove() {
    indexed_table::table! {
        struct RowTable of Row {
            foo: i32,
            bar: String,
        }
        indices {
            idx_foo => foo: i32,
            idx_bar => bar: String,
        }
    }

    let mut table = RowTable::new();
    let key = table.insert(Row { foo: 42, bar: "answer".to_string() });

    assert_eq!(table.idx_foo().find_one(&42), Some(key));
    assert_eq!(table.idx_bar().find_one(&"answer".to_string()), Some(key));

    let removed = table.remove(key);
    assert!(removed.is_some());
    assert!(table.idx_foo().find_one(&42).is_none());
    assert!(table.idx_bar().find_one(&"answer".to_string()).is_none());
}

#[test]
fn table_macro_handles_duplicate_index_values() {
    indexed_table::table! {
        struct DupTable of Row {
            foo: i32,
            bar: String,
        }
        indices {
            idx_foo => foo: i32,
        }
    }

    let mut table = DupTable::new();
    let key_a = table.insert(Row { foo: 7, bar: "a".to_string() });
    let key_b = table.insert(Row { foo: 7, bar: "b".to_string() });

    let keys = table.idx_foo().find_all(&7).unwrap();
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&key_a));
    assert!(keys.contains(&key_b));
}

#[test]
fn table_macro_rows_accessor_exposes_storage() {
    indexed_table::table! {
        struct AccessTable of Row {
            foo: i32,
            bar: String,
        }
        indices {
            idx_foo => foo: i32,
        }
    }

    let mut table = AccessTable::new();
    let key = table.insert(Row { foo: 9, bar: "nine".to_string() });
    let row = table.rows().get(key).unwrap();
    assert_eq!(row.foo, 9);
    assert_eq!(row.bar, "nine");
}

#[test]
fn table_macro_update_refreshes_indices() {
    indexed_table::table! {
        struct UpdateTable of Row {
            foo: i32,
            bar: String,
        }
        indices {
            idx_foo => foo: i32,
            idx_bar => bar: String,
        }
    }

    let mut table = UpdateTable::new();
    let key = table.insert(Row { foo: 1, bar: "one".to_string() });

    let updated = table.update(key, |row| {
        row.foo = 2;
        row.bar = "two".to_string();
    });
    assert_eq!(updated, Some(()));

    assert!(table.idx_foo().find_one(&1).is_none());
    assert!(table.idx_bar().find_one(&"one".to_string()).is_none());
    assert_eq!(table.idx_foo().find_one(&2), Some(key));
    assert_eq!(table.idx_bar().find_one(&"two".to_string()), Some(key));
}

#[test]
fn table_macro_update_unindexed_field_keeps_indices() {
    indexed_table::table! {
        struct PartialIndexTable of Row {
            foo: i32,
            bar: String,
        }
        indices {
            idx_foo => foo: i32,
        }
    }

    let mut table = PartialIndexTable::new();
    let key = table.insert(Row { foo: 5, bar: "five".to_string() });

    let updated = table.update(key, |row| {
        row.bar = "cinco".to_string();
    });
    assert_eq!(updated, Some(()));

    assert_eq!(table.idx_foo().find_one(&5), Some(key));
}
