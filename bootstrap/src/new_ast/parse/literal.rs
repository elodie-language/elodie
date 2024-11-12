use crate::new_ast::parse::node::LiteralNode;
use crate::new_ast::parse::Parser;

impl Parser {
    pub(crate) fn literal(&mut self) -> crate::new_ast::parse::Result<LiteralNode> {
        todo!()
    }
}

#[cfg(test)]
mod tests {}