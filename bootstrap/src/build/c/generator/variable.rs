use bigdecimal::ToPrimitive;

use crate::build::c;
use crate::build::c::{CallFunctionStatement, CallFunctionStatementResult, CodeExpression, Statement, VariableExpression};
use crate::build::c::generator::Generator;
use crate::build::c::generator::scope::Storage;
use crate::common::GetString;
use crate::common::node::Node::{LiteralBoolean, LiteralFloat4, LiteralFloat8, LiteralInt1, LiteralInt16, LiteralInt2, LiteralInt4, LiteralInt8, LiteralNumber, LiteralString, LiteralUint1, LiteralUint16, LiteralUint2, LiteralUint4, LiteralUint8};
use crate::ir::{IrAccessVariableNode, IrDeclareVariableNode, IrLiteralBooleanNode, IrLiteralFloat4Node, IrLiteralFloat8Node, IrLiteralInt16Node, IrLiteralInt1Node, IrLiteralInt2Node, IrLiteralInt4Node, IrLiteralInt8Node, IrLiteralNumberNode, IrLiteralStringNode, IrLiteralUint16Node, IrLiteralUint1Node, IrLiteralUint2Node, IrLiteralUint4Node, IrLiteralUint8Node};

impl Generator {
    pub(crate) fn access_variable(
        &mut self,
        node: &IrAccessVariableNode,
    ) -> c::generator::Result<VariableExpression> {
        let variable = self.symbol_table.variable(node.variable);

        Ok(VariableExpression {
            variable: variable.to_string(&self.string_table),
        })
    }

    pub(crate) fn declare_variable(
        &mut self,
        node: &IrDeclareVariableNode,
    ) -> c::generator::Result<()> {
        let variable = self.symbol_table.variable(node.variable).to_string(&self.string_table);

        self.scope.push_local_variable(variable.clone(), Storage::Memory);

        if let LiteralString(IrLiteralStringNode { value }) = &node.value.node() {
            let value = self.string_table.get_string(value);

            self.statements().push(
                Statement::CallFunction(CallFunctionStatement {
                    function: "val_str_new_from_c_str".to_string(),
                    arguments: Box::new([
                        c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                        c::Expression::Literal(c::LiteralExpression::String(
                            c::LiteralStringExpression {
                                value,
                            },
                        )),
                    ]),
                    result: Some(CallFunctionStatementResult {
                        identifier: variable,
                        r#type: "struct val_str *".to_string(),
                    }),
                })
            );

            Ok(())
        } else if let LiteralNumber(IrLiteralNumberNode { value }) = &node.value.node {
            self.statements().push(
                Statement::CallFunction(CallFunctionStatement {
                    function: "val_num_new_from_double".to_string(),
                    arguments: Box::new([
                        c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                        c::Expression::Literal(c::LiteralExpression::Float8(
                            c::LiteralFloat8Expression {
                                value: value.to_f64().unwrap(),
                            },
                        )),
                    ]),
                    result: Some(CallFunctionStatementResult {
                        identifier: variable,
                        r#type: "struct val_num *".to_string(),
                    }),
                })
            );

            Ok(())
        } else if let LiteralFloat4(IrLiteralFloat4Node { value }) = &node.value.node {
            self.statements().push(
                Statement::CallFunction(CallFunctionStatement {
                    function: "val_f4_new".to_string(),
                    arguments: Box::new([
                        c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                        c::Expression::Literal(c::LiteralExpression::Float4(
                            c::LiteralFloat4Expression {
                                value: value.clone(),
                            },
                        )),
                    ]),
                    result: Some(CallFunctionStatementResult {
                        identifier: variable,
                        r#type: "struct val_f4 *".to_string(),
                    }),
                })
            );

            Ok(())
        } else if let LiteralFloat8(IrLiteralFloat8Node { value }) = &node.value.node {
            self.statements().push(
                Statement::CallFunction(CallFunctionStatement {
                    function: "val_f8_new".to_string(),
                    arguments: Box::new([
                        c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                        c::Expression::Literal(c::LiteralExpression::Float8(
                            c::LiteralFloat8Expression {
                                value: value.clone(),
                            },
                        )),
                    ]),
                    result: Some(CallFunctionStatementResult {
                        identifier: variable,
                        r#type: "struct val_f8 *".to_string(),
                    }),
                })
            );

            Ok(())
        } else if let LiteralInt1(IrLiteralInt1Node { value }) = &node.value.node {
            self.statements().push(
                Statement::CallFunction(CallFunctionStatement {
                    function: "val_i1_new".to_string(),
                    arguments: Box::new([
                        c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                        c::Expression::Literal(c::LiteralExpression::Int1(
                            c::LiteralInt1Expression {
                                value: value.clone(),
                            },
                        )),
                    ]),
                    result: Some(CallFunctionStatementResult {
                        identifier: variable,
                        r#type: "struct val_i1 *".to_string(),
                    }),
                })
            );

            Ok(())
        } else if let LiteralInt2(IrLiteralInt2Node { value }) = &node.value.node {
            self.statements().push(
                Statement::CallFunction(CallFunctionStatement {
                    function: "val_i2_new".to_string(),
                    arguments: Box::new([
                        c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                        c::Expression::Literal(c::LiteralExpression::Int2(
                            c::LiteralInt2Expression {
                                value: value.clone(),
                            },
                        )),
                    ]),
                    result: Some(CallFunctionStatementResult {
                        identifier: variable,
                        r#type: "struct val_i2 *".to_string(),
                    }),
                })
            );

            Ok(())
        } else if let LiteralInt4(IrLiteralInt4Node { value }) = &node.value.node {
            self.statements().push(
                Statement::CallFunction(CallFunctionStatement {
                    function: "val_i4_new".to_string(),
                    arguments: Box::new([
                        c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                        c::Expression::Literal(c::LiteralExpression::Int4(
                            c::LiteralInt4Expression {
                                value: value.clone(),
                            },
                        )),
                    ]),
                    result: Some(CallFunctionStatementResult {
                        identifier: variable,
                        r#type: "struct val_i4 *".to_string(),
                    }),
                })
            );

            Ok(())
        } else if let LiteralInt8(IrLiteralInt8Node { value }) = &node.value.node {
            self.statements().push(
                Statement::CallFunction(CallFunctionStatement {
                    function: "val_i8_new".to_string(),
                    arguments: Box::new([
                        c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                        c::Expression::Literal(c::LiteralExpression::Int8(
                            c::LiteralInt8Expression {
                                value: value.clone(),
                            },
                        )),
                    ]),
                    result: Some(CallFunctionStatementResult {
                        identifier: variable,
                        r#type: "struct val_i8 *".to_string(),
                    }),
                })
            );

            Ok(())
        } else if let LiteralInt16(IrLiteralInt16Node { value }) = &node.value.node {
            self.statements().push(
                Statement::CallFunction(CallFunctionStatement {
                    function: "val_i16_new".to_string(),
                    arguments: Box::new([
                        c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                        c::Expression::Literal(c::LiteralExpression::Int16(
                            c::LiteralInt16Expression {
                                value: value.clone(),
                            },
                        )),
                    ]),
                    result: Some(CallFunctionStatementResult {
                        identifier: variable,
                        r#type: "struct val_i16 *".to_string(),
                    }),
                })
            );

            Ok(())
        } else if let LiteralUint1(IrLiteralUint1Node { value }) = &node.value.node {
            self.statements().push(
                Statement::CallFunction(CallFunctionStatement {
                    function: "val_u1_new".to_string(),
                    arguments: Box::new([
                        c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                        c::Expression::Literal(c::LiteralExpression::Uint1(
                            c::LiteralUint1Expression {
                                value: value.clone(),
                            },
                        )),
                    ]),
                    result: Some(CallFunctionStatementResult {
                        identifier: variable,
                        r#type: "struct val_u1 *".to_string(),
                    }),
                })
            );

            Ok(())
        } else if let LiteralUint2(IrLiteralUint2Node { value }) = &node.value.node {
            self.statements().push(
                Statement::CallFunction(CallFunctionStatement {
                    function: "val_u2_new".to_string(),
                    arguments: Box::new([
                        c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                        c::Expression::Literal(c::LiteralExpression::Uint2(
                            c::LiteralUint2Expression {
                                value: value.clone(),
                            },
                        )),
                    ]),
                    result: Some(CallFunctionStatementResult {
                        identifier: variable,
                        r#type: "struct val_u2 *".to_string(),
                    }),
                })
            );

            Ok(())
        } else if let LiteralUint4(IrLiteralUint4Node { value }) = &node.value.node {
            self.statements().push(
                Statement::CallFunction(CallFunctionStatement {
                    function: "val_u4_new".to_string(),
                    arguments: Box::new([
                        c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                        c::Expression::Literal(c::LiteralExpression::Uint4(
                            c::LiteralUint4Expression {
                                value: value.clone(),
                            },
                        )),
                    ]),
                    result: Some(CallFunctionStatementResult {
                        identifier: variable,
                        r#type: "struct val_u4 *".to_string(),
                    }),
                })
            );

            Ok(())
        } else if let LiteralUint8(IrLiteralUint8Node { value }) = &node.value.node {
            self.statements().push(
                Statement::CallFunction(CallFunctionStatement {
                    function: "val_u8_new".to_string(),
                    arguments: Box::new([
                        c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                        c::Expression::Literal(c::LiteralExpression::Uint8(
                            c::LiteralUint8Expression {
                                value: value.clone(),
                            },
                        )),
                    ]),
                    result: Some(CallFunctionStatementResult {
                        identifier: variable,
                        r#type: "struct val_u8 *".to_string(),
                    }),
                })
            );

            Ok(())
        } else if let LiteralUint16(IrLiteralUint16Node { value }) = &node.value.node {
            self.statements().push(
                Statement::CallFunction(CallFunctionStatement {
                    function: "val_u16_new".to_string(),
                    arguments: Box::new([
                        c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                        c::Expression::Literal(c::LiteralExpression::Uint16(
                            c::LiteralUint16Expression {
                                value: value.clone(),
                            },
                        )),
                    ]),
                    result: Some(CallFunctionStatementResult {
                        identifier: variable,
                        r#type: "struct val_u16 *".to_string(),
                    }),
                })
            );

            Ok(())
        } else if let LiteralBoolean(IrLiteralBooleanNode { value }) = &node.value.node {
            self.statements().push(
                Statement::CallFunction(CallFunctionStatement {
                    function: "val_bool_new".to_string(),
                    arguments: Box::new([
                        c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                        c::Expression::Literal(c::LiteralExpression::Bool(
                            c::LiteralBooleanExpression {
                                value: value.clone(),
                            },
                        )),
                    ]),
                    result: Some(CallFunctionStatementResult {
                        identifier: variable,
                        r#type: "struct val_bool *".to_string(),
                    }),
                })
            );

            Ok(())
        } else {
            unimplemented!("{node:#?}")
        }
    }
}