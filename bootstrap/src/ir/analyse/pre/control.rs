use std::cell::RefCell;
use std::ops::Deref;

use crate::common::Inferred;
use crate::common::node::Node::If;
use crate::frontend::ast::AstIfNode;
use crate::ir::analyse::{TypeBlockNode, TypedTreeNode, TypeIfNode};
use crate::ir::analyse::pre::Pre;

impl<'a> Pre<'a> {
    pub(crate) fn r#if(&mut self, node: &AstIfNode) -> crate::ir::analyse::Result<TypedTreeNode> {
        let condition = Box::new(self.node(node.condition.deref())?);

        self.scope.enter();
        let mut then_body = vec![];
        for node in &node.then.nodes {
            then_body.push(self.node(node.deref())?)
        }
        self.scope.leave();

        self.scope.enter();
        let otherwise = if node.otherwise.is_some() {
            let mut otherwise_body = vec![];
            for node in &node.otherwise.as_ref().unwrap().nodes {
                otherwise_body.push(self.node(node)?)
            }
            Some(RefCell::new(TypeBlockNode {
                nodes: otherwise_body.into_boxed_slice(),
            }))
        } else {
            None
        };
        self.scope.leave();

        Ok(TypedTreeNode::new(
            If(TypeIfNode {
                condition,
                then: RefCell::new(TypeBlockNode { nodes: then_body.into_boxed_slice() }),
                otherwise,
            }),
            self.span(),
            Inferred::Unit, // FIXME
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::common::{Context, Inferred, SymbolId};
    use crate::common::node::CompareOperator;
    use crate::frontend::ast_from_str;
    use crate::ir::analyse::prepare;

    #[test]
    fn nested_if() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, r#"
        let x = true
        if x == true {
            if x != false { }
        }

        "#).unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 2);

        let result = &typed[1];
        let outer = result.as_if();
        assert!(outer.otherwise.is_none());

        let compare = outer.condition.as_compare();
        let left = compare.left.as_access_variable();
        assert_eq!(left.variable, SymbolId(1));
        assert_eq!(compare.operator, CompareOperator::Equal);
        let right = compare.right.as_literal_boolean();
        assert_eq!(right.value, true);


        let inner = outer.then.borrow();
        let inner = inner.deref();
        assert_eq!(inner.nodes.len(), 1);

        let inner = inner.nodes[0].as_if();
        let compare = inner.condition.as_compare();
        let left = compare.left.as_access_variable();
        assert_eq!(left.variable, SymbolId(1));
        assert_eq!(compare.operator, CompareOperator::NotEqual);
        let right = compare.right.as_literal_boolean();
        assert_eq!(right.value, false);


        assert_eq!(result.inferred, Inferred::Unit);
    }
}