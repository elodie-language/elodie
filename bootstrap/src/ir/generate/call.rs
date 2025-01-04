use std::rc::Rc;

use crate::common::{Span, SymbolId, TypeId};
use crate::common::node::Node::CallFunctionOfPackage;
use crate::ir::analyse::TypeCallFunctionOfPackageNode;
use crate::ir::generate::Generator;
use crate::ir::node::{IrCallFunctionOfPackageNode, IrTreeNode};

impl<'a> Generator<'a> {
    pub(crate) fn call_function_of_package(&mut self, node: &TypeCallFunctionOfPackageNode, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        let mut arguments = vec![];
        for arg in &node.arguments {
            arguments.push(Rc::new(self.node(arg)?))
        }

        let result_type = TypeId::UNIT; // FIXME derive

        Ok(IrTreeNode::new(
            CallFunctionOfPackage(IrCallFunctionOfPackageNode {
                package: SymbolId(1),
                function: SymbolId(2),
                arguments: arguments.into_boxed_slice(),
            }),
            span,
            result_type,
        ))
    }
}