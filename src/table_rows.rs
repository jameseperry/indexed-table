use crate::Key;

/// Row storage for a table; does not manage indices.
pub struct TableRows<R> {
    rows: slotmap::DenseSlotMap<Key, R>,
}

impl<R> TableRows<R> {
    pub fn new() -> Self {
        Self { rows: slotmap::DenseSlotMap::new() }
    }

    pub fn insert(&mut self, row: R) -> Key {
        self.rows.insert(row)
    }

    pub fn remove(&mut self, key: Key) -> Option<R> {
        self.rows.remove(key)
    }

    pub fn get(&self, key: Key) -> Option<&R> {
        self.rows.get(key)
    }

    pub fn get_mut(&mut self, key: Key) -> Option<&mut R> {
        self.rows.get_mut(key)
    }

    pub fn iter(&self) -> slotmap::dense::Iter<'_, Key, R> {
        self.rows.iter()
    }
}
