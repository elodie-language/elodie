use crate::ast::parse::{BlockNode, PackageDeclarationNode, Parser};
use crate::ast::token::KeywordToken;
use crate::ast::token::OperatorToken::{CloseCurly, OpenCurly};

impl Parser {
    pub(crate) fn parse_package_declaration(&mut self) -> crate::ast::parse::Result<PackageDeclarationNode> {
        let fun_token = self.consume_keyword(KeywordToken::Package)?;
        let identifier = self.parse_identifier()?;
        self.consume_operator(OpenCurly)?;

        self.consume_operator(CloseCurly)?;

        Ok(PackageDeclarationNode {
            token: fun_token,
            identifier,
            block: BlockNode{ nodes: vec![] },
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::lex::lex;
    use crate::ast::parse::parse;

    #[test]
    fn empty_package() {
        let tokens = lex("package magic{ }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result.nodes[0].as_package_declaration();
        assert_eq!(node.identifier.value(), "magic");
        assert_eq!(node.block.nodes, vec![]);
    }
}