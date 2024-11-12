use crate::new_ast::parse::node::LiteralNode;
use crate::new_ast::parse::Parser;

impl Parser {
    pub(crate) fn literal(&mut self) -> crate::new_ast::parse::Result<LiteralNode> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::new_ast::lex::lex;
    use crate::new_ast::parse::parse;

    #[test]
    fn number_42() {
        let tokens = lex("42").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);
        todo!()
    }
}