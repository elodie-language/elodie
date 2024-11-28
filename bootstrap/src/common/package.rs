use std::ops::Deref;
use crate::common::{StringCache, StringCacheIdx};

#[derive(Clone, Debug)]
pub struct PackagePath {
    pub segments: Box<[StringCacheIdx]>,
}

impl From<Vec<StringCacheIdx>> for PackagePath {
    fn from(value: Vec<StringCacheIdx>) -> Self {
        Self {
            segments: value.into_boxed_slice()
        }
    }
}

impl PackagePath {
    pub fn first(&self) -> Option<StringCacheIdx> {
        self.segments.first().cloned()
    }

    pub fn pop(&self) -> Self {
        let new_segments = if self.segments.is_empty() {
            Box::new([])
        } else {
            self.segments[1..].to_vec().into_boxed_slice()
        };
        Self {
            segments: new_segments,
        }
    }

    pub fn to_strs<'a>(&self, cache: &'a StringCache) -> Box<[&'a str]>{
        self.segments.deref().into_iter().map(|s| cache.get(s)).collect()
    }
}