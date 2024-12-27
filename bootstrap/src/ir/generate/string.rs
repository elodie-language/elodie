use std::rc::Rc;

use crate::common::{Span, WithSpan};
use crate::common::node::Node;
use crate::common::node::Node::InterpolateString;
use crate::ir::{IrAccessVariableNode, IrInterpolateStringNode, IrTreeNode};
use crate::ir::analyse::{TypedTreeNode, TypeInterpolateStringNode};
use crate::ir::generate::Generator;

impl<'a> Generator<'a> {
    pub(crate) fn interpolate_string(&mut self, node: &TypeInterpolateStringNode, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        let mut nodes = Vec::with_capacity(node.nodes.len());
        for node in &node.nodes {
            nodes.push(Rc::new(self.convert_to_string(node)?))
        }

        Ok(IrTreeNode::new(
            InterpolateString(IrInterpolateStringNode { nodes: nodes.into_boxed_slice() }),
            span,
            self.type_table.type_id_string(),
        ))
    }

    pub(crate) fn convert_to_string(&mut self, node: &TypedTreeNode) -> crate::ir::generate::Result<IrTreeNode> {
        match node.node() {
            Node::AccessVariable(access_variable) => Ok(IrTreeNode::new(
                Node::AccessVariable(IrAccessVariableNode {
                    variable: access_variable.symbol
                }),
                node.span(),
                self.type_table.type_id_string(),
            )),
            _ => unimplemented!()
        }
    }
}