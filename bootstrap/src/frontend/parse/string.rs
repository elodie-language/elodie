use regex::Regex;

use crate::frontend::lex::lex;
use crate::frontend::lex::token::{LiteralToken, TextSpan, Token, TokenKind};
use crate::frontend::parse::{LiteralNode, LiteralStringNode, Node, parse, Parser, StringInterpolationNode};

impl<'a> Parser<'a> {
    pub(crate) fn parse_string(&mut self) -> crate::frontend::parse::Result<Node> {
        let token = self.consume_literal(LiteralToken::String)?;
        let value = self.ctx.get_str(token.value());
        if value.contains("${") {
            let parts = Self::extract_and_split_interpolations(value);
            let mut nodes = Vec::with_capacity(parts.len());

            for part in parts {
                if part.starts_with("${") {
                    let interest = &part[2..part.len() - 1];
                    let lexed = lex(self.ctx, interest).unwrap();
                    let parsed = parse(self.ctx, lexed)?;
                    nodes.extend(parsed);
                } else {
                    let token = Token {
                        kind: TokenKind::Literal(LiteralToken::String),
                        span: TextSpan {
                            start: token.span.start.clone(),
                            end: token.span.end.clone(),
                            value: self.ctx.string_table.insert(part.as_str()),
                        },
                    };

                    nodes.push(Node::Literal(LiteralNode::String(LiteralStringNode(token))))
                }
            }

            return Ok(Node::StringInterpolation(StringInterpolationNode { token, nodes }));
        }
        return Ok(Node::Literal(LiteralNode::String(LiteralStringNode(token))));
    }

    fn extract_and_split_interpolations(input: &str) -> Vec<String> {
        let re = Regex::new(r"\$\{(.*?)}").unwrap();

        let mut result = Vec::new();
        let mut last_index = 0;
        for cap in re.captures_iter(input) {
            let full_match = cap.get(0).unwrap();
            let expression = cap.get(1).unwrap().as_str();

            if full_match.start() > last_index {
                result.push(input[last_index..full_match.start()].to_string());
            }
            result.push(format!("${{{}}}", expression));
            last_index = full_match.end();
        }

        if last_index < input.len() {
            result.push(input[last_index..].to_string());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Context;
    use crate::frontend::lex::lex;
    use crate::frontend::parse::{InfixNode, InfixOperator, parse, StringInterpolationNode, TupleNode};
    use crate::frontend::parse::node::LiteralNode;
    use crate::frontend::parse::node::Node::Literal;
    use crate::frontend::parse::Node::StringInterpolation;

    #[test]
    fn string_literal() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "'Elodie'").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Literal(LiteralNode::String(node)) = &result[0] else { panic!() };
        assert_eq!(ctx.get_str(node.value()), "Elodie");
    }

    #[test]
    fn interpolation_with_single_identifier() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "'The value is: ${value}'").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let StringInterpolation(StringInterpolationNode { nodes, .. }) = &result[0] else { panic!() };
        assert_eq!(nodes.len(), 2);

        let Literal(LiteralNode::String(node)) = &nodes[0] else { panic!() };
        assert_eq!(ctx.get_str(node.value()), "The value is: ");

        let node = nodes[1].as_identifier();
        assert_eq!(ctx.get_str(node.value()), "value")
    }

    #[test]
    fn interpolation_with_number() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "'The value is: ${9924}'").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let StringInterpolation(StringInterpolationNode { nodes, .. }) = &result[0] else { panic!() };
        assert_eq!(nodes.len(), 2);

        let Literal(LiteralNode::String(node)) = &nodes[0] else { panic!() };
        assert_eq!(ctx.get_str(node.value()), "The value is: ");

        let Literal(LiteralNode::Number(node)) = &nodes[1] else { panic!() };
        assert_eq!(ctx.get_str(node.value()), "9924");
    }

    #[test]
    fn interpolation_with_bool() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "'The value is: ${true}'").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let StringInterpolation(StringInterpolationNode { nodes, .. }) = &result[0] else { panic!() };
        assert_eq!(nodes.len(), 2);

        let Literal(LiteralNode::String(node)) = &nodes[0] else { panic!() };
        assert_eq!(ctx.get_str(node.value()), "The value is: ");

        let Literal(LiteralNode::Boolean(node)) = &nodes[1] else { panic!() };
        assert_eq!(node.value(), true);
    }

    #[test]
    fn interpolation_with_function_call() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "'The value is: ${some_function('elodie')}'").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let StringInterpolation(StringInterpolationNode { nodes, .. }) = &result[0] else { panic!() };
        assert_eq!(nodes.len(), 2);

        let Literal(LiteralNode::String(node)) = &nodes[0] else { panic!() };
        assert_eq!(ctx.get_str(node.value()), "The value is: ");

        let InfixNode { left, operator, right } = &nodes[1].as_infix();
        let identifier = left.as_identifier();
        assert_eq!(ctx.get_str(identifier.value()), "some_function");

        let InfixOperator::Call(_) = operator else { panic!() };

        let TupleNode { nodes, .. } = right.as_tuple();
        assert_eq!(nodes.len(), 1);

        let Some(Literal(LiteralNode::String(arg_1))) = &nodes.first() else { panic!() };
        assert_eq!(ctx.get_str(arg_1.value()), "elodie");
    }
}