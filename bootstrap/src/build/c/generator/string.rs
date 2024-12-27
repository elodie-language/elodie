use crate::build::c;
use crate::build::c::{DeclareVariableStatement, Indent, VariableExpression};
use crate::build::c::generator::Generator;
use crate::ir::IrInterpolateStringNode;

impl Generator {

    pub(crate) fn interpolate_string(&mut self, node: &IrInterpolateStringNode) -> c::generator::Result<VariableExpression> {

        self.current_function_statements().push(
            c::Statement::DeclareVariable(DeclareVariableStatement {
                indent: Indent::none(),
                variable: "arg_2".to_string(),
                r#type: "const char *".to_string(),
                expression: c::Expression::Variable(VariableExpression { indent: Indent::none(), identifier: "arg_1".to_string() }),
            })
        );

        Ok(VariableExpression {
            indent: Indent::none(),
            identifier: "arg_2".to_string(),
        })
    }
}