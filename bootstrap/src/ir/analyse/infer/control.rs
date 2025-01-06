use crate::common::Inferred;
use crate::common::node::Node;
use crate::ir::analyse::infer::Inferrer;
use crate::ir::analyse::TypedTreeNode;

impl<'a> Inferrer<'a> {
    pub(crate) fn r#break(&mut self, node: &mut TypedTreeNode) -> crate::ir::analyse::Result<()> {
        if let Node::BreakLoop(r#break) = &mut node.node {
            if let Some(value) = &r#break.node {
                // unimplemented!("{value:#?}")
                node.inferred = value.inferred.clone();
            }
            Ok(())
        } else {
            panic!("not break")
        }
    }

    pub(crate) fn r#loop(&mut self, node: &mut TypedTreeNode) -> crate::ir::analyse::Result<()> {
        if let Node::Loop(r#loop) = &mut node.node {
            self.block_node(&mut r#loop.nodes.get_mut())?;

            node.inferred = r#loop.nodes.borrow().nodes.last().map(|n| n.inferred.clone()).unwrap_or(Inferred::Unit);
            Ok(())
        } else {
            panic!("not loop")
        }
    }

    pub(crate) fn r#if(&mut self, node: &mut TypedTreeNode) -> crate::ir::analyse::Result<()> {
        if let Node::If(r#if) = &mut node.node {
            self.block_node(&mut r#if.then.get_mut())?;

            let inferred = r#if.then.borrow().nodes.last().map(|n| n.inferred.clone()).unwrap_or(Inferred::Unit);

            node.inferred = inferred;

            if let Some(cell) = &mut r#if.otherwise {
                self.block_node(cell.get_mut())?;
            }

            Ok(())
        } else {
            panic!("not if")
        }
    }
}