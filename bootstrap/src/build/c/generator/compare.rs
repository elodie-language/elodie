use Node::{AccessVariable, LiteralBoolean, LiteralNumber};

use crate::build::c;
use crate::build::c::{CallFunctionExpression, CodeExpression, CompareExpression, Expression, VariableExpression};
use crate::build::c::generator::Generator;
use crate::common::node::Node;
use crate::common::TypeId;
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

                if TypeId::BOOLEAN == variable.type_id.unwrap() {
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

                if TypeId::BOOLEAN == variable.type_id.unwrap() {
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
            (LiteralNumber(_), LiteralNumber(_)) => {
                Ok(Expression::Compare(
                    CompareExpression {
                        left: Box::new(left),
                        operator: node.operator.clone(),
                        right: Box::new(right),
                    }
                ))
            }
            (AccessVariable(left), AccessVariable(right)) => {
                let left = self.symbol_table.variable(left.variable);
                let right = self.symbol_table.variable(right.variable);

                match (left.type_id, right.type_id) {
                    (Some(TypeId::NUMBER), Some(TypeId::NUMBER)) => {
                        Ok(Expression::CallFunction(
                            CallFunctionExpression {
                                function: "val_num_cmp".to_string(),
                                arguments: Box::new([
                                    c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                    Expression::Variable(VariableExpression { variable: left.to_string(&self.string_table), cast: None }),
                                    Expression::compare_operator(&node.operator),
                                    Expression::Variable(VariableExpression { variable: right.to_string(&self.string_table), cast: None }),
                                ]),
                            }
                        ))
                    }
                    (Some(TypeId::FLOAT4), Some(TypeId::FLOAT4)) => {
                        Ok(Expression::CallFunction(
                            CallFunctionExpression {
                                function: "val_f4_cmp".to_string(),
                                arguments: Box::new([
                                    c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                    Expression::Variable(VariableExpression { variable: left.to_string(&self.string_table), cast: None }),
                                    Expression::compare_operator(&node.operator),
                                    Expression::Variable(VariableExpression { variable: right.to_string(&self.string_table), cast: None }),
                                ]),
                            }
                        ))
                    }
                    (Some(TypeId::FLOAT8), Some(TypeId::FLOAT8)) => {
                        Ok(Expression::CallFunction(
                            CallFunctionExpression {
                                function: "val_f8_cmp".to_string(),
                                arguments: Box::new([
                                    c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                    Expression::Variable(VariableExpression { variable: left.to_string(&self.string_table), cast: None }),
                                    Expression::compare_operator(&node.operator),
                                    Expression::Variable(VariableExpression { variable: right.to_string(&self.string_table), cast: None }),
                                ]),
                            }
                        ))
                    }
                    (Some(TypeId::INT1), Some(TypeId::INT1)) => {
                        Ok(Expression::CallFunction(
                            CallFunctionExpression {
                                function: "val_i1_cmp".to_string(),
                                arguments: Box::new([
                                    c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                    Expression::Variable(VariableExpression { variable: left.to_string(&self.string_table), cast: None }),
                                    Expression::compare_operator(&node.operator),
                                    Expression::Variable(VariableExpression { variable: right.to_string(&self.string_table), cast: None }),
                                ]),
                            }
                        ))
                    }
                    (Some(TypeId::INT2), Some(TypeId::INT2)) => {
                        Ok(Expression::CallFunction(
                            CallFunctionExpression {
                                function: "val_i2_cmp".to_string(),
                                arguments: Box::new([
                                    c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                    Expression::Variable(VariableExpression { variable: left.to_string(&self.string_table), cast: None }),
                                    Expression::compare_operator(&node.operator),
                                    Expression::Variable(VariableExpression { variable: right.to_string(&self.string_table), cast: None }),
                                ]),
                            }
                        ))
                    }
                    (Some(TypeId::INT4), Some(TypeId::INT4)) => {
                        Ok(Expression::CallFunction(
                            CallFunctionExpression {
                                function: "val_i4_cmp".to_string(),
                                arguments: Box::new([
                                    c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                    Expression::Variable(VariableExpression { variable: left.to_string(&self.string_table), cast: None }),
                                    Expression::compare_operator(&node.operator),
                                    Expression::Variable(VariableExpression { variable: right.to_string(&self.string_table), cast: None }),
                                ]),
                            }
                        ))
                    }
                    (Some(TypeId::INT8), Some(TypeId::INT8)) => {
                        Ok(Expression::CallFunction(
                            CallFunctionExpression {
                                function: "val_i8_cmp".to_string(),
                                arguments: Box::new([
                                    c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                    Expression::Variable(VariableExpression { variable: left.to_string(&self.string_table), cast: None }),
                                    Expression::compare_operator(&node.operator),
                                    Expression::Variable(VariableExpression { variable: right.to_string(&self.string_table), cast: None }),
                                ]),
                            }
                        ))
                    }
                    (Some(TypeId::INT16), Some(TypeId::INT16)) => {
                        Ok(Expression::CallFunction(
                            CallFunctionExpression {
                                function: "val_i16_cmp".to_string(),
                                arguments: Box::new([
                                    c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                    Expression::Variable(VariableExpression { variable: left.to_string(&self.string_table), cast: None }),
                                    Expression::compare_operator(&node.operator),
                                    Expression::Variable(VariableExpression { variable: right.to_string(&self.string_table), cast: None }),
                                ]),
                            }
                        ))
                    }
                    (Some(TypeId::UINT1), Some(TypeId::UINT1)) => {
                        Ok(Expression::CallFunction(
                            CallFunctionExpression {
                                function: "val_u1_cmp".to_string(),
                                arguments: Box::new([
                                    c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                    Expression::Variable(VariableExpression { variable: left.to_string(&self.string_table), cast: None }),
                                    Expression::compare_operator(&node.operator),
                                    Expression::Variable(VariableExpression { variable: right.to_string(&self.string_table), cast: None }),
                                ]),
                            }
                        ))
                    }
                    (Some(TypeId::UINT2), Some(TypeId::UINT2)) => {
                        Ok(Expression::CallFunction(
                            CallFunctionExpression {
                                function: "val_u2_cmp".to_string(),
                                arguments: Box::new([
                                    c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                    Expression::Variable(VariableExpression { variable: left.to_string(&self.string_table), cast: None }),
                                    Expression::compare_operator(&node.operator),
                                    Expression::Variable(VariableExpression { variable: right.to_string(&self.string_table), cast: None }),
                                ]),
                            }
                        ))
                    }
                    (Some(TypeId::UINT4), Some(TypeId::UINT4)) => {
                        Ok(Expression::CallFunction(
                            CallFunctionExpression {
                                function: "val_u4_cmp".to_string(),
                                arguments: Box::new([
                                    c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                    Expression::Variable(VariableExpression { variable: left.to_string(&self.string_table), cast: None }),
                                    Expression::compare_operator(&node.operator),
                                    Expression::Variable(VariableExpression { variable: right.to_string(&self.string_table), cast: None }),
                                ]),
                            }
                        ))
                    }
                    (Some(TypeId::UINT8), Some(TypeId::UINT8)) => {
                        Ok(Expression::CallFunction(
                            CallFunctionExpression {
                                function: "val_u8_cmp".to_string(),
                                arguments: Box::new([
                                    c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                    Expression::Variable(VariableExpression { variable: left.to_string(&self.string_table), cast: None }),
                                    Expression::compare_operator(&node.operator),
                                    Expression::Variable(VariableExpression { variable: right.to_string(&self.string_table), cast: None }),
                                ]),
                            }
                        ))
                    }
                    (Some(TypeId::UINT16), Some(TypeId::UINT16)) => {
                        Ok(Expression::CallFunction(
                            CallFunctionExpression {
                                function: "val_u16_cmp".to_string(),
                                arguments: Box::new([
                                    c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                    Expression::Variable(VariableExpression { variable: left.to_string(&self.string_table), cast: None }),
                                    Expression::compare_operator(&node.operator),
                                    Expression::Variable(VariableExpression { variable: right.to_string(&self.string_table), cast: None }),
                                ]),
                            }
                        ))
                    }
                    _ => unimplemented!()
                }
            }
            _ => unimplemented!()
        }
    }
}