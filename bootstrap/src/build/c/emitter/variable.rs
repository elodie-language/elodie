use crate::build::c::emitter::Emitter;
use crate::build::c::{
    DeclareArrayStatement, DeclareVariableStatement, VariableExpression,
};

impl Emitter {
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
        self.token(statement.identifier.as_str());
        self.token("=");
        self.expression(&statement.expression);
        self.line(";");
    }

    pub(crate) fn variable(&mut self, expression: &VariableExpression) {
        self.str(expression.identifier.as_str());
    }
}
