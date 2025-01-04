use crate::common::{Inferred, Span, SymbolId};
use crate::common::node::Node;
use crate::frontend::ast::AstCallFunctionOfPackageNode;
use crate::ir::analyse::{TypeCallFunctionOfPackageNode, TypedTreeNode};
use crate::ir::analyse::pre::Pre;

impl<'a> Pre<'a> {
    pub(crate) fn call_function_of_package(&mut self, node: &AstCallFunctionOfPackageNode) -> crate::ir::analyse::Result<TypedTreeNode> {
        // get package
        // get function from package

        let mut arguments = vec![];
        for arg in &node.arguments {
            arguments.push(
                self.node(arg)?
            )
        }

        Ok(TypedTreeNode::new(
            Node::CallFunctionOfPackage(
                TypeCallFunctionOfPackageNode {
                    package: SymbolId(2),
                    function: SymbolId(3),
                    arguments: arguments.into_boxed_slice(),
                }
            ),
            self.span(),
            Inferred::Unknown,
        ))
    }
}