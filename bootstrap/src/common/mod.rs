pub use package::PackagePath;
pub use strings::{StringCache, StringCacheIdx};
pub use util::*;

mod util;
mod strings;
mod package;


pub struct Context {
    pub string_cache: StringCache,
}

impl Context {
    pub fn new() -> Self {
        Self { string_cache: StringCache::new() }
    }
}

impl Context {
    pub fn get_str(&self, idx: StringCacheIdx) -> &str { self.string_cache.get(idx) }
}