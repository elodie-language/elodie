use std::ops::Index;

use crate::common::context::Context;
use crate::common::node::Node::LiteralNumber;
use crate::frontend::ast::SPAN_NOT_IMPLEMENTED;
use crate::ir::analyse::{TypedAst, TypeLiteralNumberNode};
use crate::ir::Ir;
use crate::ir::node::{IrLiteralNumberNode, IrTreeNode};
use crate::ir::TypeId;

mod literal;

pub(crate) fn generate(ctx: &mut Context, typed: TypedAst) -> crate::ir::Result<Ir> {
    let TypeLiteralNumberNode { value } = typed.nodes.index(0).as_literal_number() else { panic!() };
    Ok(
        Ir {
            nodes: vec![
                IrTreeNode::new(
                    LiteralNumber(IrLiteralNumberNode { value: value.clone() }), SPAN_NOT_IMPLEMENTED.clone(), TypeId(1))
            ]
        }
    )
}
