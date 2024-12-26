use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StringTableId(pub usize);

impl AsRef<StringTableId> for StringTableId {
    fn as_ref(&self) -> &StringTableId {
        &self
    }
}

#[derive(Debug)]
pub struct StringTable {
    index: HashMap<Rc<str>, StringTableId>,
    values: Vec<Rc<str>>,
}

impl StringTable {
    pub fn new() -> StringTable {
        StringTable {
            index: HashMap::new(),
            values: Vec::new(),
        }
    }

    pub fn push_str(&mut self, string: &str) -> StringTableId {
        match self.index.entry(string.into()) {
            std::collections::hash_map::Entry::Occupied(entry) => *entry.get(),
            std::collections::hash_map::Entry::Vacant(entry) => {
                let idx = StringTableId(self.values.len());
                self.values.push(entry.key().clone());
                entry.insert(idx);
                idx
            }
        }
    }

    pub fn get(&self, idx: impl AsRef<StringTableId>) -> &str {
        &self
            .values
            .get(idx.as_ref().0)
            .expect("StringIdx out of bounds")
    }

    pub fn as_string(&self, idx: impl AsRef<StringTableId>) -> String { self.get(idx).to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut map = StringTable::new();
        let idx1 = map.push_str("hello");
        let idx2 = map.push_str("world");
        let idx3 = map.push_str("hello");

        assert_eq!(idx1, idx3);
        assert_ne!(idx1, idx2);
        assert_eq!(map.get(idx1), "hello");
        assert_eq!(map.get(idx1), "hello");
        assert_eq!(map.get(idx2), "world");
    }

    #[test]
    #[should_panic(expected = "StringIdx out of bounds")]
    fn get_out_of_bounds() {
        let map = StringTable::new();
        map.get(StringTableId(0));
    }
}
