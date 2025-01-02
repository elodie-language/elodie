use crate::build::c;
use crate::build::c::{CallFunctionStatement, CallFunctionStatementResult, CodeExpression, Indent, Statement, VariableExpression};
use crate::build::c::generator::Generator;
use crate::common::GetString;
use crate::common::node::Node::LiteralString;
use crate::ir::{IrAccessVariableNode, IrDeclareVariableNode, IrLiteralStringNode};

impl Generator {
    pub(crate) fn access_variable(
        &mut self,
        node: &IrAccessVariableNode,
    ) -> c::generator::Result<VariableExpression> {
        let variable = self.symbol_table.variable(node.variable);

        Ok(VariableExpression {
            indent: Indent::none(),
            variable: variable.to_string(&self.string_table),
        })
    }

    pub(crate) fn declare_variable(
        &mut self,
        node: &IrDeclareVariableNode,
    ) -> c::generator::Result<()> {
        let variable = self.symbol_table.variable(node.variable).to_string(&self.string_table);

        if let LiteralString(IrLiteralStringNode { value }) = &node.value.node() {
            let value = self.string_table.get_string(value);

            self.statements().push(
                Statement::CallFunction(CallFunctionStatement {
                    indent: Indent::none(),
                    function: "val_str_new_from_c_str".to_string(),
                    arguments: Box::new([
                        c::Expression::Code(CodeExpression { indent: Indent::none(), code: "MEM(tm)".to_string() }),
                        c::Expression::Literal(c::LiteralExpression::String(
                            c::LiteralStringExpression {
                                indent: Indent::none(),
                                value,
                            },
                        )),
                    ]),
                    result: Some(CallFunctionStatementResult {
                        indent: Indent::none(),
                        identifier: variable,
                        r#type: "struct val_str *".to_string(),
                    }),
                })
            );

            Ok(())
        } else {
            unimplemented!("{node:#?}")
        }
    }
}