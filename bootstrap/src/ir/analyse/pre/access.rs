use crate::common::node::Node::AccessVariable;
use crate::common::SymbolName;
use crate::frontend::ast::AstAccessVariableNode;
use crate::ir::analyse::{TypeAccessVariableNode, TypedTreeNode};
use crate::ir::analyse::pre::Pre;

impl<'a> Pre<'a> {
    pub(crate) fn access_variable(
        &mut self,
        node: &AstAccessVariableNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let variable = self.variable_get(SymbolName::from(&node.variable))?;

        Ok(TypedTreeNode::new(
            AccessVariable(TypeAccessVariableNode {
                variable: variable.id.clone()
            }),
            self.span(),
            variable.inferred.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{Inferred, SymbolId};
    use crate::common::Context;
    use crate::frontend::ast_from_str;
    use crate::ir::analyse::{prepare, UndefinedError};
    use crate::ir::analyse::Error::Undefined;

    #[test]
    fn access_variable() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, r#"
            let some_thing_else = false
            let value = true
            value
        "#).unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 3);

        let result = &typed[2];
        let inner = result.as_access_variable();
        assert_eq!(inner.variable, SymbolId(2));

        assert_eq!(result.inferred, Inferred::Boolean);
    }

    #[test]
    fn variable_does_not_exists() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, r#"
            value
        "#).unwrap();
        let result = prepare(&mut ctx, ast);
        assert!(result.is_err());

        let error = result.err().unwrap();
        let Undefined(UndefinedError::UndefinedVariable { variable, .. }) = error else { panic!() };

        assert_eq!(variable, "value")
    }
}