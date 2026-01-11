use indexed_table as it;

// Row type used by the generated table (defined at module scope so macro can see it)
struct Blah {
    foo: i64,
    bar: String,
}

#[test]
fn macro_generates_table_with_indices() {
    // Invoke the macro (indices block is mandatory per updated macro)
    indexed_table::table! {
        struct BlahTable of Blah {
            foo: i64,
            bar: String,
        }
        indices {
            idx_foo => foo: i64,
            idx_bar => bar: String,
        }
    }

    let mut table = BlahTable::new();

    let k = table.insert(Blah { foo: 7, bar: "seven".into() });
    // indices should contain the key for foo and bar values
    let found_foo = table.idx_foo.find(&7);
    assert!(found_foo.is_some());
    assert_eq!(table.rows.get(k).unwrap().foo, 7);

    let found_bar = table.idx_bar.find(&"seven".to_string());
    assert!(found_bar.is_some());
    assert_eq!(table.rows.get(k).unwrap().bar, "seven");

    // remove and ensure indices are updated
    let _ = table.remove(k);
    assert!(table.idx_foo.find(&7).is_none());
    assert!(table.idx_bar.find(&"seven".to_string()).is_none());
}
