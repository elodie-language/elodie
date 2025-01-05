use crate::build::c;
use crate::build::c::{CallFunctionExpression, CodeExpression, Expression, VariableExpression};
use crate::build::c::generator::Generator;
use crate::common::node::Node::AccessVariable;
use crate::common::TypeId;
use crate::ir::IrCalculateNode;

impl Generator {
    pub(crate) fn calculate(
        &mut self,
        node: &IrCalculateNode,
    ) -> c::generator::Result<Expression> {
        match (node.left.as_ref().node(), node.right.as_ref().node()) {
            (AccessVariable(left_variable), AccessVariable(right_variable)) => {
                let left_variable = self.symbol_table.variable(left_variable.variable);
                let right_variable = self.symbol_table.variable(right_variable.variable);

                if left_variable.type_id == Some(TypeId::NUMBER) {
                    Ok(Expression::CallFunction(
                        CallFunctionExpression {
                            function: "val_num_calc".to_string(),
                            arguments: Box::new([
                                c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                Expression::Variable(VariableExpression { variable: left_variable.to_string(&self.string_table), cast: None }),
                                Expression::calculate_operator(&node.operator),
                                Expression::Variable(VariableExpression { variable: right_variable.to_string(&self.string_table), cast: None }),
                            ]),
                        }
                    ))
                } else if left_variable.type_id == Some(TypeId::FLOAT4) {
                    Ok(Expression::CallFunction(
                        CallFunctionExpression {
                            function: "val_f4_calc".to_string(),
                            arguments: Box::new([
                                c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                Expression::Variable(VariableExpression { variable: left_variable.to_string(&self.string_table), cast: None }),
                                Expression::calculate_operator(&node.operator),
                                Expression::Variable(VariableExpression { variable: right_variable.to_string(&self.string_table), cast: None }),
                            ]),
                        }
                    ))
                } else if left_variable.type_id == Some(TypeId::FLOAT8) {
                    Ok(Expression::CallFunction(
                        CallFunctionExpression {
                            function: "val_f8_calc".to_string(),
                            arguments: Box::new([
                                c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                Expression::Variable(VariableExpression { variable: left_variable.to_string(&self.string_table), cast: None }),
                                Expression::calculate_operator(&node.operator),
                                Expression::Variable(VariableExpression { variable: right_variable.to_string(&self.string_table), cast: None }),
                            ]),
                        }
                    ))
                } else if left_variable.type_id == Some(TypeId::INT1) {
                    Ok(Expression::CallFunction(
                        CallFunctionExpression {
                            function: "val_i1_calc".to_string(),
                            arguments: Box::new([
                                c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                Expression::Variable(VariableExpression { variable: left_variable.to_string(&self.string_table), cast: None }),
                                Expression::calculate_operator(&node.operator),
                                Expression::Variable(VariableExpression { variable: right_variable.to_string(&self.string_table), cast: None }),
                            ]),
                        }
                    ))
                } else if left_variable.type_id == Some(TypeId::INT2) {
                    Ok(Expression::CallFunction(
                        CallFunctionExpression {
                            function: "val_i2_calc".to_string(),
                            arguments: Box::new([
                                c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                Expression::Variable(VariableExpression { variable: left_variable.to_string(&self.string_table), cast: None }),
                                Expression::calculate_operator(&node.operator),
                                Expression::Variable(VariableExpression { variable: right_variable.to_string(&self.string_table), cast: None }),
                            ]),
                        }
                    ))
                } else if left_variable.type_id == Some(TypeId::INT4) {
                    Ok(Expression::CallFunction(
                        CallFunctionExpression {
                            function: "val_i4_calc".to_string(),
                            arguments: Box::new([
                                c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                Expression::Variable(VariableExpression { variable: left_variable.to_string(&self.string_table), cast: None }),
                                Expression::calculate_operator(&node.operator),
                                Expression::Variable(VariableExpression { variable: right_variable.to_string(&self.string_table), cast: None }),
                            ]),
                        }
                    ))
                } else if left_variable.type_id == Some(TypeId::INT8) {
                    Ok(Expression::CallFunction(
                        CallFunctionExpression {
                            function: "val_i8_calc".to_string(),
                            arguments: Box::new([
                                c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                Expression::Variable(VariableExpression { variable: left_variable.to_string(&self.string_table), cast: None }),
                                Expression::calculate_operator(&node.operator),
                                Expression::Variable(VariableExpression { variable: right_variable.to_string(&self.string_table), cast: None }),
                            ]),
                        }
                    ))
                } else if left_variable.type_id == Some(TypeId::INT16) {
                    Ok(Expression::CallFunction(
                        CallFunctionExpression {
                            function: "val_i16_calc".to_string(),
                            arguments: Box::new([
                                c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                Expression::Variable(VariableExpression { variable: left_variable.to_string(&self.string_table), cast: None }),
                                Expression::calculate_operator(&node.operator),
                                Expression::Variable(VariableExpression { variable: right_variable.to_string(&self.string_table), cast: None }),
                            ]),
                        }
                    ))
                } else if left_variable.type_id == Some(TypeId::UINT1) {
                    Ok(Expression::CallFunction(
                        CallFunctionExpression {
                            function: "val_u1_calc".to_string(),
                            arguments: Box::new([
                                c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                Expression::Variable(VariableExpression { variable: left_variable.to_string(&self.string_table), cast: None }),
                                Expression::calculate_operator(&node.operator),
                                Expression::Variable(VariableExpression { variable: right_variable.to_string(&self.string_table), cast: None }),
                            ]),
                        }
                    ))
                } else if left_variable.type_id == Some(TypeId::UINT2) {
                    Ok(Expression::CallFunction(
                        CallFunctionExpression {
                            function: "val_u2_calc".to_string(),
                            arguments: Box::new([
                                c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                Expression::Variable(VariableExpression { variable: left_variable.to_string(&self.string_table), cast: None }),
                                Expression::calculate_operator(&node.operator),
                                Expression::Variable(VariableExpression { variable: right_variable.to_string(&self.string_table), cast: None }),
                            ]),
                        }
                    ))
                } else if left_variable.type_id == Some(TypeId::UINT4) {
                    Ok(Expression::CallFunction(
                        CallFunctionExpression {
                            function: "val_u4_calc".to_string(),
                            arguments: Box::new([
                                c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                Expression::Variable(VariableExpression { variable: left_variable.to_string(&self.string_table), cast: None }),
                                Expression::calculate_operator(&node.operator),
                                Expression::Variable(VariableExpression { variable: right_variable.to_string(&self.string_table), cast: None }),
                            ]),
                        }
                    ))
                } else if left_variable.type_id == Some(TypeId::UINT8) {
                    Ok(Expression::CallFunction(
                        CallFunctionExpression {
                            function: "val_u8_calc".to_string(),
                            arguments: Box::new([
                                c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                Expression::Variable(VariableExpression { variable: left_variable.to_string(&self.string_table), cast: None }),
                                Expression::calculate_operator(&node.operator),
                                Expression::Variable(VariableExpression { variable: right_variable.to_string(&self.string_table), cast: None }),
                            ]),
                        }
                    ))
                } else if left_variable.type_id == Some(TypeId::UINT16) {
                    Ok(Expression::CallFunction(
                        CallFunctionExpression {
                            function: "val_u16_calc".to_string(),
                            arguments: Box::new([
                                c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                                Expression::Variable(VariableExpression { variable: left_variable.to_string(&self.string_table), cast: None }),
                                Expression::calculate_operator(&node.operator),
                                Expression::Variable(VariableExpression { variable: right_variable.to_string(&self.string_table), cast: None }),
                            ]),
                        }
                    ))
                } else {
                    unimplemented!()
                }
            }
            _ => unimplemented!()
        }
    }
}