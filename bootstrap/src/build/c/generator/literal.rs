use bigdecimal::ToPrimitive;

use crate::build::c;
use crate::build::c::{CallFunctionStatement, CallFunctionStatementResult, CodeExpression, Expression, LiteralBooleanExpression, LiteralFloat8Expression, LiteralExpression, Statement, VariableExpression};
use crate::build::c::generator::Generator;
use crate::build::c::generator::scope::Storage;
use crate::common::GetString;
use crate::ir::{IrLiteralBooleanNode, IrLiteralNumberNode, IrLiteralStringNode};

impl Generator {
    pub(crate) fn literal_bool(
        &mut self,
        node: &IrLiteralBooleanNode,
    ) -> c::generator::Result<Expression> {
        Ok(Expression::Literal(LiteralExpression::Bool(LiteralBooleanExpression {
            value: node.value,
        })))
    }

    pub(crate) fn literal_number(
        &mut self,
        node: &IrLiteralNumberNode,
    ) -> crate::build::c::generator::Result<Expression> {

        // FIXME becomes big decimal
        Ok(Expression::Literal(LiteralExpression::Float8(LiteralFloat8Expression {
            value: node.value.to_f64().unwrap(),
        })))
    }

    pub(crate) fn literal_string(
        &mut self,
        node: &IrLiteralStringNode,
    ) -> c::generator::Result<Expression> {
        // Ok(LiteralExpression::String(LiteralStringExpression {
        //     indent: Indent::none(),
        //     value: self.string_table.get(node.value).to_string(),
        // }))
        let temp = self.scope.push_temp(Storage::Memory);
        let value = self.string_table.get_string(node.value);
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
                    identifier: temp.to_string(),
                    r#type: "struct val_str *".to_string(),
                }),
            }));

        Ok(c::Expression::Variable(VariableExpression { variable: temp.to_string(), cast: None }))
    }
}
