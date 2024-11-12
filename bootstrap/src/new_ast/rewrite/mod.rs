use crate::new_ast::ast::SourceFile;
use crate::new_ast::parse::node::RootNode;

#[derive(Debug)]
pub enum Error {}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

pub(crate) struct Rewriter {}

impl Rewriter {
    pub(crate) fn rewrite(node: RootNode) -> Result<SourceFile> {
        todo!()
    }
}