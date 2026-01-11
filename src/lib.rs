//! indexed-table — tiny in-memory relational-like table (skeleton)
//!
//! This crate provides a small `TableIndex` implementation and a macro to
//! generate typed table wrappers backed by `slotmap::DenseSlotMap`.

use slotmap::DefaultKey;

/// Key type used across the crate (re-exported from `slotmap`).
pub type Key = DefaultKey;

/// Trait for indices that can be updated when rows are inserted/removed.
///
/// This is primarily used by the table macro and is not typically implemented
/// directly by users.
pub trait Index<R> {
    fn insert_row(&mut self, row: &R, key: Key);
    fn remove_row(&mut self, row: &R, key: Key);
}

pub mod table_rows;
pub mod table_index;
pub mod table;

pub use table_rows::TableRows;
pub use table_index::TableIndex;
