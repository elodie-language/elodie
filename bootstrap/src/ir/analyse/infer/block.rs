use crate::common::node::Node;
use crate::ir::analyse::{TypeBlockNode, TypedTreeNode};
use crate::ir::analyse::infer::Inferrer;

impl<'a> Inferrer<'a> {
    pub(crate) fn block(&mut self, node: &mut TypedTreeNode) -> crate::ir::analyse::Result<()> {
        if let Node::Block(block) = &mut node.node {
            // self.scope.enter();

            for node in &mut block.nodes {
                self.node(node)?;
            }

            // self.scope.leave();
            Ok(())
        } else {
            panic!("not block")
        }
    }

    pub(crate) fn block_node(&mut self, node: &mut TypeBlockNode) -> crate::ir::analyse::Result<()> {
        // self.scope.enter();

        for node in &mut node.nodes {
            self.node(node)?;
        }

        // self.scope.leave();
        Ok(())
    }
}