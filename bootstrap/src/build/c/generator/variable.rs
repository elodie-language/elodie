use crate::build::c;
use crate::build::c::{DeclareVariableStatement, Indent, Statement};
use crate::build::c::generator::Generator;
use crate::common::GetString;
use crate::common::node::Node::LiteralString;
use crate::ir::{IrDeclareVariableNode, IrLiteralStringNode};

impl Generator {
    pub(crate) fn declare_variable(
        &mut self,
        node: &IrDeclareVariableNode,
    ) -> c::generator::Result<()> {
        let variable = self.symbol_table.variable(node.variable).to_string(&self.string_table);

        if let LiteralString(IrLiteralStringNode { value }) = &node.value.node() {
            let value = self.string_table.get_string(value);
            self.statements().push(
                Statement::DeclareVariable(DeclareVariableStatement {
                    indent: Indent::none(),
                    variable,
                    r#type: "const char *".to_string(),
                    expression: c::Expression::Literal(c::LiteralExpression::String(
                        c::LiteralStringExpression {
                            indent: Indent::none(),
                            value,
                        },
                    )),
                }));

            Ok(())
        } else {
            unimplemented!("{node:#?}")
        }
    }
}