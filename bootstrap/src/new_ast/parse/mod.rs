use std::collections::HashMap;

use crate::core::token::{Operator, Token, TokenKind};
use crate::new_ast::parser::node::RootNode;
use crate::new_ast::parser::precedence::Precedence;

pub(crate) mod precedence;
pub(crate) mod node;

pub(crate) struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    precedence_map: HashMap<TokenKind, Precedence>,
}

impl<'a> Parser<'a> {

    pub(crate) fn parse(tokens: &'a [Token]) -> crate::new_ast::Result<RootNode> {
        todo!()
    }

    fn new(tokens: &'a [Token]) -> Self {
        let mut precedence_map = HashMap::new();

        precedence_map.insert(TokenKind::Operator(Operator::Arrow), Precedence::Assignment);

        precedence_map.insert(TokenKind::Operator(Operator::DoubleEqual), Precedence::Equality);
        precedence_map.insert(TokenKind::Operator(Operator::BangEqual), Precedence::Equality);

        precedence_map.insert(TokenKind::Operator(Operator::LeftAngle), Precedence::Comparison);
        precedence_map.insert(TokenKind::Operator(Operator::LeftAngleEqual), Precedence::Comparison);
        precedence_map.insert(TokenKind::Operator(Operator::RightAngle), Precedence::Comparison);
        precedence_map.insert(TokenKind::Operator(Operator::RightAngleEqual), Precedence::Comparison);

        precedence_map.insert(TokenKind::Operator(Operator::Plus), Precedence::Term);
        precedence_map.insert(TokenKind::Operator(Operator::Minus), Precedence::Term);

        precedence_map.insert(TokenKind::Operator(Operator::Asterisk), Precedence::Factor);
        precedence_map.insert(TokenKind::Operator(Operator::Slash), Precedence::Factor);
        precedence_map.insert(TokenKind::Operator(Operator::Percent), Precedence::Factor);

        precedence_map.insert(TokenKind::Operator(Operator::OpenParen), Precedence::Call);
        precedence_map.insert(TokenKind::Operator(Operator::Dot), Precedence::Primary);
        precedence_map.insert(TokenKind::Operator(Operator::DoubleColon), Precedence::Primary);

        Self {
            tokens,
            current: 0,
            precedence_map,
        }
    }
}
