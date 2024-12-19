use crate::frontend::ast;
use crate::ir::infer::InferredType;
use crate::ir::symbol::SymbolId;

#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    DeclareVariable(DeclareVariableNode<'a>),
    Literal(LiteralNode<'a>),
}

impl<'a> Node<'a> {
    pub fn inferred_type(&mut self) -> InferredType {
        match self {
            Node::DeclareVariable(DeclareVariableNode { inferred_type, .. })
            | Node::Literal(LiteralNode::Boolean(LiteralBooleanNode { inferred_type, .. }))
            | Node::Literal(LiteralNode::Number(LiteralNumberNode { inferred_type, .. }))
            | Node::Literal(LiteralNode::String(LiteralStringNode { inferred_type, .. })) => inferred_type.clone()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DeclareVariableNode<'a> {
    pub ast: &'a ast::DeclareVariableNode,
    pub symbol: SymbolId,
    pub node: Box<Node<'a>>,
    pub inferred_type: InferredType,
}

#[derive(Debug, PartialEq)]
pub enum LiteralNode<'a> {
    Boolean(LiteralBooleanNode<'a>),
    Number(LiteralNumberNode<'a>),
    String(LiteralStringNode<'a>),
}


#[derive(Debug, PartialEq)]
pub struct LiteralBooleanNode<'a> {
    pub ast: &'a ast::LiteralBooleanNode,
    pub inferred_type: InferredType,
}

#[derive(Debug, PartialEq)]
pub struct LiteralNumberNode<'a> {
    pub ast: &'a ast::LiteralNumberNode,
    pub inferred_type: InferredType,
}

#[derive(Debug, PartialEq)]
pub struct LiteralStringNode<'a> {
    pub ast: &'a ast::LiteralStringNode,
    pub inferred_type: InferredType,
}