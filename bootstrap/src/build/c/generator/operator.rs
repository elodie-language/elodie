use Node::{AccessVariable, LiteralBoolean};

use crate::build::c;
use crate::build::c::{CallFunctionExpression, CompareExpression, Expression};
use crate::build::c::generator::Generator;
use crate::common::node::Node;
use crate::ir::IrCompareNode;

impl Generator {
    pub(crate) fn compare(
        &mut self,
        node: &IrCompareNode,
    ) -> c::generator::Result<Expression> {
        let left = self.expression(node.left.as_ref())?;
        let right = self.expression(node.right.as_ref())?;

        match (node.left.as_ref().node(), node.right.as_ref().node()) {
            (LiteralBoolean(_), AccessVariable(variable)) => {
                let variable = self.symbol_table.variable(variable.variable);

                if self.type_table.type_id_boolean() == variable.type_id.unwrap() {
                    Ok(Expression::CallFunction(
                        CallFunctionExpression {
                            function: "val_bool_cmp_lit".to_string(),
                            arguments: Box::new([
                                right,
                                Expression::compare_operator(&node.operator),
                                left,
                            ]),
                        }
                    ))
                } else {
                    unimplemented!()
                }
            }

            (AccessVariable(variable), LiteralBoolean(_)) => {
                let variable = self.symbol_table.variable(variable.variable);

                if self.type_table.type_id_boolean() == variable.type_id.unwrap() {
                    Ok(Expression::CallFunction(
                        CallFunctionExpression {
                            function: "val_bool_cmp_lit".to_string(),
                            arguments: Box::new([
                                left,
                                Expression::compare_operator(&node.operator),
                                right,
                            ]),
                        }
                    ))
                } else {
                    unimplemented!()
                }
            }

            (Node::LiteralNumber(_), Node::LiteralNumber(_)) => {
                Ok(Expression::Compare(
                    CompareExpression {
                        left: Box::new(left),
                        operator: node.operator.clone(),
                        right: Box::new(right),
                    }
                ))
            }
            _ => unimplemented!()
        }
    }
}