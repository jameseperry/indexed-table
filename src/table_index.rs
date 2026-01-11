use crate::Key;
use crate::Index;
use std::collections::HashMap;
use std::hash::Hash;

/// `TableIndex` maps a field value `F` to one or more `Key`s for `TableData<R>`.
pub struct TableIndex<R, F>
where
    F: Eq + Hash + Clone,
{
    map: HashMap<F, Vec<Key>>,
    extractor: Box<dyn Fn(&R) -> F>,
}

impl<R, F> TableIndex<R, F>
where
    F: Eq + Hash + Clone,
{
    pub fn new(extractor: impl Fn(&R) -> F + 'static) -> Self {
        Self { map: HashMap::new(), extractor: Box::new(extractor) }
    }

    pub fn insert(&mut self, row: &R, key: Key) {
        let field = (self.extractor)(row);
        self.map.entry(field).or_default().push(key);
    }

    /// Returns the first key for the field, if any (duplicates allowed).
    pub fn find_one(&self, field: &F) -> Option<Key> {
        self.map.get(field).and_then(|v| v.first().cloned())
    }

    pub fn find_all(&self, field: &F) -> Option<Vec<Key>> {
        self.map.get(field).map(|v| v.clone())
    }

    pub fn remove(&mut self, field: &F, key: Key) {
        if let Some(vec) = self.map.get_mut(field) {
            vec.retain(|k| *k != key);
            if vec.is_empty() {
                self.map.remove(field);
            }
        }
    }

    pub fn remove_by_row(&mut self, row: &R, key: Key) {
        let field = (self.extractor)(row);
        self.remove(&field, key);
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

impl<R, F> Index<R> for TableIndex<R, F>
where
    F: Eq + Hash + Clone,
{
    fn insert_row(&mut self, row: &R, key: Key) {
        self.insert(row, key)
    }

    fn remove_row(&mut self, row: &R, key: Key) {
        self.remove_by_row(row, key)
    }
}
