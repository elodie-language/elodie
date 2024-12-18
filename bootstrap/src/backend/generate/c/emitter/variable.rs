use crate::backend::generate::c::emitter::Emitter;
use crate::backend::generate::c::{
    DeclareArrayStatement, DeclareVariableStatement, VariableExpression,
};

impl Emitter {
    pub(crate) fn emit_declare_array(&mut self, statement: &DeclareArrayStatement) {
        self.emit_token(statement.r#type.as_str());
        self.emit_str(statement.identifier.as_str());
        self.emit_str("[");
        self.emit_str(statement.size.to_string().as_str());
        self.emit_str("]");
        self.emit_line(";");
    }

    pub(crate) fn emit_declare_variable(&mut self, statement: &DeclareVariableStatement) {
        self.emit_token(statement.r#type.as_str());
        self.emit_token(statement.identifier.as_str());
        self.emit_token("=");
        self.emit_expression(&statement.expression);
        self.emit_line(";");
    }

    pub(crate) fn emit_variable(&mut self, expression: &VariableExpression) {
        self.emit_str(expression.identifier.as_str());
    }
}
