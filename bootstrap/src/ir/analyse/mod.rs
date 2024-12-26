use std::ops::Index;

pub use node::*;

use crate::common::{Span, StringTable, Type, TypeId, TypeTable};
use crate::common::context::Context;
use crate::frontend::Ast;
use crate::ir::analyse::infer::Inferrer;
use crate::ir::analyse::pre::Pre;

mod infer;
mod node;
mod pre;
mod scope;

#[derive(Debug, Clone, PartialEq)]
pub enum InferredType {
    Unknown,

    Boolean,
    Function(Box<[InferredType]>, Box<InferredType>),
    Number,
    Package,
    String,
    Tuple(Box<[InferredType]>),
    Type(TypeId),

    OneOf(Box<[InferredType]>),
    AllOf(Box<[InferredType]>),
}

impl InferredType {
    pub fn to_string(&self, string_table: &StringTable) -> String {
        match self {
            InferredType::Boolean => "Boolean".to_string(),
            InferredType::Number => "Number".to_string(),
            InferredType::String => "String".to_string(),
            _ => unimplemented!("{self:#?}")
        }
    }
}

impl Index<InferredType> for TypeTable {
    type Output = Type;
    fn index(&self, index: InferredType) -> &Self::Output {
        let type_id = match index {
            InferredType::Boolean => self.type_id_boolean(),
            InferredType::Number => self.type_id_number(),
            InferredType::String => self.type_id_string(),
            _ => unimplemented!()
        };

        &self.index(type_id)
    }
}


#[derive(Debug, PartialEq)]
pub enum Error {
    TypeMissMatch(TypeMissMatchError)
}

#[derive(Debug, PartialEq)]
pub enum TypeMissMatchError {
    DeclaredTypeMissMatch { expected: String, got: String, span: Span }
}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct TypedAst {
    pub nodes: Vec<TypedTreeNode>,
}

impl Index<usize> for TypedAst {
    type Output = TypedTreeNode;
    fn index(&self, index: usize) -> &Self::Output {
        self.nodes.index(index)
    }
}

pub(crate) fn prepare(ctx: &mut Context, ast: Ast) -> Result<TypedAst> {
    let mut nodes = Pre::new(ctx).process(ast)?;
    Ok(TypedAst { nodes })
}

pub(crate) fn infer(ctx: &mut Context, ast: TypedAst) -> Result<TypedAst> {
    let mut nodes = ast.nodes;
    Inferrer::new(ctx).infer_nodes(&mut nodes)?;
    Ok(TypedAst { nodes })
}

pub(crate) fn analyse(ctx: &mut Context, ast: Ast) -> Result<TypedAst> {
    let prepared = prepare(ctx, ast)?;
    infer(ctx, prepared)
}
