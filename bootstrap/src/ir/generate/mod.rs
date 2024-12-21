use std::ops::Index;

use crate::common::node::Node::LiteralNumber;
use crate::frontend::ast::SPAN_NOT_IMPLEMENTED;
use crate::ir::{Context, TypeId};
use crate::ir::analyse::{Analyse, AnalyseLiteralNumberNode};
use crate::ir::Ir;
use crate::ir::node::{IrLiteralNumberNode, IrTreeNode};

mod literal;

pub(crate) fn generate(ctx: &mut Context, analysed: Analyse) -> crate::ir::Result<Ir> {
    let AnalyseLiteralNumberNode { value } = analysed.nodes.index(0).as_literal_number() else { panic!() };
    Ok(
        Ir {
            nodes: vec![
                IrTreeNode::new(LiteralNumber(IrLiteralNumberNode { value: value.clone(), value_type: TypeId(1) }), SPAN_NOT_IMPLEMENTED.clone())
            ]
        }
    )
}
