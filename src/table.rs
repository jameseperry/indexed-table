/// Macro to generate a typed table wrapper with `TableRows<R>` and
/// read-only accessors for `TableIndex<R, F>` indices.
///
/// Usage examples:
/// table! {
///     pub struct BlahTable of Blah {
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
    ($vis:vis struct $name:ident of $row:ty { $($field:ident : $fty:ty),* $(,)? }
     indices { $($idx_name:ident => $idx_field:ident : $idx_fty:ty),+ $(,)? }
    ) => {
        #[allow(non_camel_case_types)]
        $vis struct $name {
            rows: $crate::TableRows<$row>,
            $($idx_name: $crate::TableIndex<$row, $idx_fty>),*
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    rows: $crate::TableRows::new(),
                    $($idx_name: $crate::TableIndex::new(|r: &$row| r.$idx_field.clone())),*
                }
            }

            /// Insert a row into the underlying `DenseSlotMap` (indices updated).
            pub fn insert(&mut self, row: $row) -> $crate::Key {
                let key = self.rows.insert(row);
                if let Some(r) = self.rows.get(key) {
                    $($crate::Index::insert_row(&mut self.$idx_name, r, key);)*
                }
                key
            }

            /// Remove a row and update indices.
            pub fn remove(&mut self, key: $crate::Key) -> Option<$row> {
                if let Some(r) = self.rows.get(key) {
                    $($crate::Index::remove_row(&mut self.$idx_name, r, key);)*
                }
                self.rows.remove(key)
            }

            pub fn rows(&self) -> &$crate::TableRows<$row> {
                &self.rows
            }

            /// Read-only accessor for the generated index.
            $(pub fn $idx_name(&self) -> &$crate::TableIndex<$row, $idx_fty> {
                &self.$idx_name
            })*
        }

    };
}
