use crate::lex::token::KeywordToken;
use crate::lex::token::KeywordToken::Export;
use crate::parse::{FromExportNode, FromNode, Parser};
use crate::parse::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_from(&mut self) -> crate::parse::Result<FromNode> {
        let token = self.consume_keyword(KeywordToken::From)?;
        let where_node = Box::new(self.parse_node(Precedence::None)?);

        let is_export = self.current()?.is_keyword(KeywordToken::Export);

        if is_export {
            let _ = self.consume_keyword(Export)?;
            let what_node = Box::new(self.parse_node(Precedence::None)?);

            return Ok(FromNode::Export(FromExportNode {
                token,
                from_node: where_node,
                what_node,
            }));
        }

        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Context;
    use crate::lex::lex;
    use crate::parse::{FromNode, LiteralNode, parse};

    #[test]
    fn export_single_package_from_local_file() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "from './io' export io").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let FromNode::Export(node) = result.nodes[0].as_from();

        let LiteralNode::String(literal) = &node.from_node.as_literal() else { panic!("not string literal") };
        assert_eq!(ctx.get_str(literal.value()), "./io");

        let identifier = node.what_node.as_identifier();
        assert_eq!(ctx.get_str(identifier.value()), "io");
    }
}