use std::hash::{Hash, Hasher};

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum DirectiveNode {
    IncludeSystemDirective(IncludeSystemDirectiveNode),
    IncludeLocalDirective(IncludeLocalDirectiveNode),
}

#[derive(Debug)]
pub struct IncludeSystemDirectiveNode {
    pub path: String,
}

impl Eq for IncludeSystemDirectiveNode {}

impl PartialEq for IncludeSystemDirectiveNode {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}

impl Hash for IncludeSystemDirectiveNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state)
    }
}

#[derive(Debug)]
pub struct IncludeLocalDirectiveNode {
    pub path: String,
}

impl Eq for IncludeLocalDirectiveNode {}

impl PartialEq for IncludeLocalDirectiveNode {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}

impl Hash for IncludeLocalDirectiveNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state)
    }
}