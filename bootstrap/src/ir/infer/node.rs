use crate::common::Symbol;
use crate::frontend::parse;
use crate::ir::infer::InferredType;

#[derive(Debug, PartialEq)]
pub enum Node {
    Let(LetNode),
    Literal(LiteralNode),
}

impl Node {
    pub fn inferred_type(&self) -> InferredType {
        match self {
            Node::Let(LetNode { inferred_type, .. })
            | Node::Literal(LiteralNode::Boolean(LiteralBooleanNode { inferred_type, .. }))
            | Node::Literal(LiteralNode::Number(LiteralNumberNode { inferred_type, .. }))
            | Node::Literal(LiteralNode::String(LiteralStringNode { inferred_type, .. })) => inferred_type.clone()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LetNode {
    pub parsed_node: parse::LetNode,
    pub symbol: Symbol,
    pub inferred_type: InferredType,
}

#[derive(Debug, PartialEq)]
pub enum LiteralNode {
    Boolean(LiteralBooleanNode),
    Number(LiteralNumberNode),
    String(LiteralStringNode),
}


#[derive(Debug, PartialEq)]
pub struct LiteralBooleanNode {
    pub parsed_node: parse::LiteralBooleanNode,
    pub inferred_type: InferredType,
}

#[derive(Debug, PartialEq)]
pub struct LiteralNumberNode {
    pub parsed_node: parse::LiteralNumberNode,
    pub inferred_type: InferredType,
}

#[derive(Debug, PartialEq)]
pub struct LiteralStringNode {
    pub parsed_node: parse::LiteralStringNode,
    pub inferred_type: InferredType,
}