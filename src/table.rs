/// Macro to generate a typed table wrapper that owns `TableData<R>` and
/// index fields named `idx_...` for `TableIndex<R, F>` indices.
///
/// Usage examples:
/// table! {
///     struct BlahTable of Blah {
///         foo: i64,
///         bar: String,
///     }
///     indices {
///         idxFoo => foo: i64,
///         idxBar => bar: String,
///     }
/// }
///

#[macro_export]
macro_rules! table {
    // indices: `idx_name => field_name : FieldType` (explicit types required)
    (struct $name:ident of $row:ty { $($field:ident : $fty:ty),* $(,)? }
     indices { $($idx_name:ident => $idx_field:ident : $idx_fty:ty),* $(,)? }
    ) => {
        #[allow(non_camel_case_types)]
        pub struct $name {
            pub rows: slotmap::DenseSlotMap<$crate::Key, $row>,
            $(pub $idx_name: $crate::TableIndex<$row, $idx_fty>),*
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    rows: slotmap::DenseSlotMap::new(),
                    $($idx_name: $crate::TableIndex::new(|r: &$row| r.$idx_field.clone())),*
                }
            }

            /// Insert a row into the underlying `DenseSlotMap` (indices updated).
            pub fn insert(&mut self, row: $row) -> $crate::Key {
                let key = self.rows.insert(row);
                if let Some(r) = self.rows.get(key) {
                    $(self.$idx_name.insert(r, key);)*
                }
                key
            }

            /// Remove a row and update indices.
            pub fn remove(&mut self, key: $crate::Key) -> Option<$row> {
                if let Some(r) = self.rows.get(key) {
                    $(self.$idx_name.remove_by_row(r, key);)*
                }
                self.rows.remove(key)
            }
        }
    };
}
