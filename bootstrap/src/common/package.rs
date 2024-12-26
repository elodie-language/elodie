use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::common::{StringTable, StringTableId};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PackageId(pub usize);

impl AsRef<PackageId> for PackageId {
    fn as_ref(&self) -> &PackageId {
        &self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PackagePath {
    pub segments: Box<[StringTableId]>,
}

impl From<Vec<StringTableId>> for PackagePath {
    fn from(value: Vec<StringTableId>) -> Self {
        Self {
            segments: value.into_boxed_slice(),
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

    pub fn to_strs<'a>(&self, cache: &'a StringTable) -> Box<[&'a str]> {
        self.segments
            .deref()
            .into_iter()
            .map(|s| cache.get(s))
            .collect()
    }
}

#[derive(Debug)]
pub struct Package {

}

#[derive(Debug)]
pub struct PackageTable {
    index: HashMap<PackagePath, PackageId>,
    values: Vec<Rc<Package>>,
}