use std::rc::Rc;

use bigdecimal::BigDecimal;

use crate::common::{Span, StringTableId, WithSpan};
use crate::ir::analyse::InferredType;
use crate::ir::symbol::SymbolId;

pub trait Analysed<T: Analysed<T>>: Clone {
    fn node(&self) -> &Inner<T>;
    fn node_mut(&mut self) -> &mut Inner<T>;
    fn node_to_owned(self) -> Inner<T>;
}

#[derive(Clone, Debug)]
pub struct AnalysedNode {
    pub inner: Inner<AnalysedNode>,
    pub span: Span,
    pub inferred_type: InferredType,
}

impl AnalysedNode {
    pub fn as_literal_boolean(&self) -> &LiteralBooleanInner {
        if let Inner::LiteralBoolean(result) = &self.inner {
            result
        } else {
            panic!("not literal boolean")
        }
    }

    pub fn as_literal_number(&self) -> &LiteralNumberInner {
        if let Inner::LiteralNumber(result) = &self.inner {
            result
        } else {
            panic!("not literal number")
        }
    }

    pub fn as_literal_string(&self) -> &LiteralStringInner {
        if let Inner::LiteralString(result) = &self.inner {
            result
        } else {
            panic!("not literal string")
        }
    }

    pub fn as_declared_variable(&self) -> &DeclareVariableInner<AnalysedNode> {
        if let Inner::DeclareVariable(result) = &self.inner {
            result
        } else {
            panic!("not declare variable")
        }
    }
}

impl AnalysedNode {
    pub fn new(inner: Inner<AnalysedNode>, span: Span, inferred_type: InferredType) -> AnalysedNode {
        AnalysedNode {
            inner,
            span,
            inferred_type,
        }
    }
}

impl Analysed<AnalysedNode> for AnalysedNode {
    fn node(&self) -> &Inner<AnalysedNode> { &self.inner }
    fn node_mut(&mut self) -> &mut Inner<AnalysedNode> { &mut self.inner }
    fn node_to_owned(self) -> Inner<AnalysedNode> { self.inner }
}

impl WithSpan for AnalysedNode {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Inner<T: Analysed<T>> {
    DeclareVariable(DeclareVariableInner<T>),
    LiteralBoolean(LiteralBooleanInner),
    LiteralNumber(LiteralNumberInner),
    LiteralString(LiteralStringInner),
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeclareVariableInner<T: Clone + Analysed<T>> {
    pub symbol: SymbolId,
    pub value: Rc<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralBooleanInner {
    pub value: bool,
}


#[derive(Debug, Clone, PartialEq)]
pub struct LiteralNumberInner {
    pub value: BigDecimal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralStringInner {
    pub value: StringTableId,
}