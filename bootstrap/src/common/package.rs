use std::ops::Deref;
use crate::common::{StringTable, StringTableId};

#[derive(Clone, Debug)]
pub struct PackagePath {
    pub segments: Box<[StringTableId]>,
}

impl From<Vec<StringTableId>> for PackagePath {
    fn from(value: Vec<StringTableId>) -> Self {
        Self {
            segments: value.into_boxed_slice()
        }
    }
}

impl PackagePath {
    pub fn first(&self) -> Option<StringTableId> {
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

    pub fn to_strs<'a>(&self, cache: &'a StringTable) -> Box<[&'a str]>{
        self.segments.deref().into_iter().map(|s| cache.get(s)).collect()
    }
}