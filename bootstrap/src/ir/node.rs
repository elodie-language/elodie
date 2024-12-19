use std::rc::Rc;
use crate::frontend::ast;
use crate::ir::{analyse, TypeId};
use crate::ir::analyse::InferredType;

#[derive(Debug, PartialEq)]
pub enum Node {
    DeclareVariable(DeclareVariableNode),
    Literal(LiteralNode),
}

#[derive(Debug, PartialEq)]
pub struct DeclareVariableNode {
    pub analysed: Rc<analyse::DeclareVariableNode>,
    pub node: Box<Node>,
    pub r#type: TypeId,
}

#[derive(Debug, PartialEq)]
pub enum LiteralNode {
    Boolean(LiteralBooleanNode),
    Number(LiteralNumberNode),
    String(LiteralStringNode),
}

#[derive(Debug, PartialEq)]
pub struct LiteralBooleanNode {
    pub analysed: Rc<ast::LiteralBooleanNode>,
    pub inferred_type: InferredType,
}

#[derive(Debug, PartialEq)]
pub struct LiteralNumberNode {
    pub analysed: Rc<ast::LiteralNumberNode>,
    pub inferred_type: InferredType,
}

#[derive(Debug, PartialEq)]
pub struct LiteralStringNode {
    pub analysed: Rc<ast::LiteralStringNode>,
    pub inferred_type: InferredType,
}