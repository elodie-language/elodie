use crate::ast::parse::Error::UnknownType;
use crate::ast::parse::node::{TypeFundamentalNode, TypeNode};
use crate::ast::parse::Parser;

impl Parser {
    pub(crate) fn parse_type(&mut self) -> crate::ast::parse::Result<TypeNode> {
        let token = self.advance()?;
        match token.value() {
            "Boolean" => Ok(TypeNode::Fundamental(TypeFundamentalNode::Boolean(token))),
            "Number" => Ok(TypeNode::Fundamental(TypeFundamentalNode::Number(token))),
            "String" => Ok(TypeNode::Fundamental(TypeFundamentalNode::String(token))),
            _ => Err(UnknownType(token))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::lex::lex;
    use crate::ast::parse::Error::UnknownType;
    use crate::ast::parse::node::{TypeFundamentalNode, TypeNode};
    use crate::ast::parse::Parser;

    #[test]
    fn not_a_type() {
        let tokens = lex("something_different").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse_type();
        let Err(UnknownType(_)) = result else { panic!() };
    }

    #[test]
    fn type_boolean() {
        let tokens = lex("Boolean").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse_type().unwrap();
        let TypeNode::Fundamental(TypeFundamentalNode::Boolean(_)) = result else { panic!() };
    }

    #[test]
    fn type_number() {
        let tokens = lex("Number").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse_type().unwrap();
        let TypeNode::Fundamental(TypeFundamentalNode::Number(_)) = result else { panic!() };
    }

    #[test]
    fn type_string() {
        let tokens = lex("String").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse_type().unwrap();
        let TypeNode::Fundamental(TypeFundamentalNode::String(_)) = result else { panic!() };
    }
}