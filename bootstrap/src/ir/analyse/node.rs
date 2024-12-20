use std::rc::Rc;

use bigdecimal::BigDecimal;

use crate::common::{Span, StringTableId, WithSpan};
use crate::ir::analyse::InferredType;
use crate::ir::symbol::SymbolId;

pub trait Analysed<T: Analysed<T>>: Clone {
    fn node(&self) -> &Node<T>;
    fn node_mut(&mut self) -> &mut Node<T>;
    fn node_to_owned(self) -> Node<T>;
}

#[derive(Clone, Debug)]
pub struct AnalysedNode {
    pub node: Node<AnalysedNode>,
    pub span: Span,
    pub inferred_type: InferredType,
}

impl AnalysedNode {
    pub fn as_literal_boolean(&self) -> &LiteralBooleanNode {
        if let Node::LiteralBoolean(result) = &self.node {
            result
        } else {
            panic!("not literal boolean")
        }
    }

    pub fn as_literal_number(&self) -> &LiteralNumberNode {
        if let Node::LiteralNumber(result) = &self.node {
            result
        } else {
            panic!("not literal number")
        }
    }

    pub fn as_literal_string(&self) -> &LiteralStringNode {
        if let Node::LiteralString(result) = &self.node {
            result
        } else {
            panic!("not literal string")
        }
    }

    pub fn as_declared_variable(&self) -> &DeclareVariableNode<AnalysedNode> {
        if let Node::DeclareVariable(result) = &self.node {
            result
        } else {
            panic!("not declare variable")
        }
    }
}

impl AnalysedNode {
    pub fn new(inner: Node<AnalysedNode>, span: Span, inferred_type: InferredType) -> AnalysedNode {
        AnalysedNode {
            node: inner,
            span,
            inferred_type,
        }
    }
}

impl Analysed<AnalysedNode> for AnalysedNode {
    fn node(&self) -> &Node<AnalysedNode> { &self.node }
    fn node_mut(&mut self) -> &mut Node<AnalysedNode> { &mut self.node }
    fn node_to_owned(self) -> Node<AnalysedNode> { self.node }
}

impl WithSpan for AnalysedNode {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node<T: Analysed<T>> {
    DeclareVariable(DeclareVariableNode<T>),
    LiteralBoolean(LiteralBooleanNode),
    LiteralNumber(LiteralNumberNode),
    LiteralString(LiteralStringNode),
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeclareVariableNode<T: Clone + Analysed<T>> {
    pub symbol: SymbolId,
    pub value: Rc<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralBooleanNode {
    pub value: bool,
}


#[derive(Debug, Clone, PartialEq)]
pub struct LiteralNumberNode {
    pub value: BigDecimal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralStringNode {
    pub value: StringTableId,
}