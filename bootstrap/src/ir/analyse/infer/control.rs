use crate::common::node::Node;
use crate::ir::analyse::infer::Inferrer;
use crate::ir::analyse::TypedTreeNode;

impl<'a> Inferrer<'a> {
    pub(crate) fn r#if(&mut self, node: &mut TypedTreeNode) -> crate::ir::analyse::Result<()> {
        if let Node::If(r#if) = &mut node.node {
            self.block_node(&mut r#if.then.get_mut())?;

            if let Some(cell) = &mut r#if.otherwise {
                self.block_node(cell.get_mut())?;
            }

            Ok(())
        } else {
            panic!("not if")
        }
    }
}