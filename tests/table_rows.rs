use indexed_table::TableRows;

#[test]
fn table_rows_insert_get_remove() {
    let mut rows = TableRows::new();
    let key_a = rows.insert(10);
    let key_b = rows.insert(20);

    assert_eq!(rows.get(key_a), Some(&10));
    assert_eq!(rows.get(key_b), Some(&20));

    let removed = rows.remove(key_a);
    assert_eq!(removed, Some(10));
    assert!(rows.get(key_a).is_none());
    assert_eq!(rows.get(key_b), Some(&20));
}

#[test]
fn table_rows_iter() {
    let mut rows = TableRows::new();
    let key_a = rows.insert(1);
    let key_b = rows.insert(2);
    let key_c = rows.insert(3);

    let mut seen_keys = Vec::new();
    let mut seen_values = Vec::new();
    for (key, value) in rows.iter() {
        seen_keys.push(key);
        seen_values.push(*value);
    }

    assert_eq!(seen_keys.len(), 3);
    assert!(seen_keys.contains(&key_a));
    assert!(seen_keys.contains(&key_b));
    assert!(seen_keys.contains(&key_c));

    seen_values.sort();
    assert_eq!(seen_values, vec![1, 2, 3]);
}
