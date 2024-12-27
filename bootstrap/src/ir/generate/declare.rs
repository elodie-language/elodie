use std::rc::Rc;

use Node::DeclareVariable;

use crate::common::node::Node;
use crate::common::Span;
use crate::ir::analyse::TypeDeclareVariableNode;
use crate::ir::generate::Generator;
use crate::ir::node::{IrDeclareVariableNode, IrTreeNode};

impl<'a> Generator<'a> {
    pub(crate) fn declare_variable(&mut self, node: &TypeDeclareVariableNode, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        let value = Rc::new(self.node(node.value.as_ref())?);

        Ok(IrTreeNode::new(
            DeclareVariable(IrDeclareVariableNode {
                variable: node.variable,
                value: value.clone(),
            }),
            span,
            value.type_id,
        ))
    }
}

mod tests {
    use bigdecimal::BigDecimal;

    use crate::common::Context;
    use crate::ir::ir_from_str;

    #[test]
    fn declare_variable_with_explicit_type() {
        let mut ctx = Context::testing();
        let ir = ir_from_str(&mut ctx, "let var: Number = 9924").unwrap();
        assert_eq!(ir.len(), 1);

        let result = &ir[0];
        let declared_variable = result.as_declare_variable();
        assert_eq!(ctx.symbol_name(declared_variable.variable), "var");

        let value = declared_variable.value.as_literal_number();
        assert_eq!(value.value, BigDecimal::from(9924))
    }
}
