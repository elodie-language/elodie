use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StringCacheIdx(pub usize);

#[derive(Debug, Clone)]
pub struct StringCache {
    indexes: HashMap<Rc<str>, StringCacheIdx>,
    values: Vec<Rc<str>>,
}

impl StringCache {
    pub fn new() -> StringCache {
        StringCache { indexes: HashMap::new(), values: Vec::new() }
    }

    pub fn insert(&mut self, string: &str) -> StringCacheIdx {
        match self.indexes.entry(string.into()) {
            std::collections::hash_map::Entry::Occupied(entry) => {
                *entry.get()
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                let idx = StringCacheIdx(self.values.len());
                self.values.push(entry.key().clone());
                entry.insert(idx);
                idx
            }
        }
    }

    pub fn get(&self, idx: StringCacheIdx) -> &str {
        &self.values.get(idx.0).expect("StringIdx out of bounds")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut map = StringCache::new();
        let idx1 = map.insert("hello");
        let idx2 = map.insert("world");
        let idx3 = map.insert("hello");

        assert_eq!(idx1, idx3);
        assert_ne!(idx1, idx2);
        assert_eq!(map.get(idx1), "hello");
        assert_eq!(map.get(idx1), "hello");
        assert_eq!(map.get(idx2), "world");
    }

    #[test]
    #[should_panic(expected = "StringIdx out of bounds")]
    fn get_out_of_bounds() {
        let map = StringCache::new();
        map.get(StringCacheIdx(0));
    }
}
