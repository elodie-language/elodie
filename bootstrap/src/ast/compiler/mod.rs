mod symbol;

use crate::ast::ast::SourceFile;
use crate::ast::parse::node::RootNode;

#[derive(Debug)]
pub enum Error {}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

pub(crate) fn from(node: RootNode) -> Result<SourceFile> {
    Ok(SourceFile {})
}
