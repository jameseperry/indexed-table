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
    rows: DenseSlotMap<DefaultKey, Blah>,
    idx_foo: TableIndex<i64>,
    idx_bar: TableIndex<String>
}
```

and appropriate update functions.