# indexed-table

Example macro usage (see `tests/macro_usage.rs`):

```rust
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
```

Generates:

```rust
struct Blah {
    foo: i64,
    bar: String,
}

struct BlahTable {
    rows: TableRows<Blah>,
    idx_foo: TableIndex<Blah, i64>,
    idx_bar: TableIndex<Blah, String>
}
```

Example usage:

```rust
let key = table.insert(Blah { foo: 7, bar: "seven".into() });
let row = table.rows().get(key).unwrap();
let found = table.idx_foo().find_one(&7);
```

and appropriate update functions.
