use crate::build::c::{AccessVariableOfStructExpression, DeclareArrayStatement, DeclareVariableStatement, VariableExpression};
use crate::build::c::emitter::Emitter;

impl Emitter {
    pub(crate) fn access_variable_of_object(&mut self, expression: &AccessVariableOfStructExpression) {
        self.str(expression.r#struct.as_str());
        self.str("->");
        self.str(expression.variable.as_str());
    }

    pub(crate) fn declare_array(&mut self, statement: &DeclareArrayStatement) {
        self.token(statement.r#type.as_str());
        self.str(statement.identifier.as_str());
        self.str("[");
        self.str(statement.size.to_string().as_str());
        self.str("]");
        self.line(";");
    }

    pub(crate) fn declare_variable(&mut self, statement: &DeclareVariableStatement) {
        self.token(statement.r#type.as_str());
        self.token(statement.variable.as_str());
        self.token("=");
        self.expression(&statement.expression);
        self.line(";");
    }

    pub(crate) fn variable(&mut self, expression: &VariableExpression) {
        self.str(expression.variable.as_str());
    }
}
