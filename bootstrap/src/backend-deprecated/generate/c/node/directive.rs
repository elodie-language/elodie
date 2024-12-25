use crate::backend::generate::c::Indent;

#[derive(Debug)]
pub enum DirectiveNode {
    IncludeSystemDirective(IncludeSystemDirectiveNode),
    IncludeLocalDirective(IncludeLocalDirectiveNode),
}

#[derive(Debug)]
pub struct IncludeSystemDirectiveNode {
    pub indent: Indent,
    pub path: String,
}

#[derive(Debug)]
pub struct IncludeLocalDirectiveNode {
    pub indent: Indent,
    pub path: String,
}
