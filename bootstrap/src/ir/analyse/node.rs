use std::rc::Rc;

use crate::frontend::ast;
use crate::ir::analyse::InferredType;
use crate::ir::symbol::SymbolId;

#[derive(Debug, PartialEq)]
pub enum Node {
    DeclareVariable(DeclareVariableNode),
    Literal(LiteralNode),
}

impl Node {
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
pub struct DeclareVariableNode {
    pub ast: Rc<ast::DeclareVariableNode>,
    pub symbol: SymbolId,
    pub node: Box<Node>,
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
    pub ast: Rc<ast::LiteralBooleanNode>,
    pub inferred_type: InferredType,
}

#[derive(Debug, PartialEq)]
pub struct LiteralNumberNode {
    pub ast: Rc<ast::LiteralNumberNode>,
    pub inferred_type: InferredType,
}

#[derive(Debug, PartialEq)]
pub struct LiteralStringNode {
    pub ast: Rc<ast::LiteralStringNode>,
    pub inferred_type: InferredType,
}