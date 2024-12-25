use crate::backend::generate::c::emitter::Emitter;
use crate::backend::generate::c::{
    DeclareStructNode, DefineStructNode, InitialiseStructExpression, InitialiseStructField,
};

impl Emitter {
    pub(crate) fn emit_declare_struct(&mut self, node: &DeclareStructNode) {
        self.emit_token("struct");
        self.emit_str(node.identifier.as_str());
        self.emit_line(";");
    }

    pub(crate) fn emit_define_struct(&mut self, node: &DefineStructNode) {
        self.emit_token("struct");
        self.emit_str(node.identifier.as_str());
        self.emit_line("{");

        for field in &node.fields {
            self.emit_token(field.ty.as_str());
            self.emit_str(field.identifier.as_str());
            self.emit_line(";");
        }

        self.emit_line("};");
    }

    pub(crate) fn emit_initialise_struct(&mut self, expression: &InitialiseStructExpression) {
        self.emit_str("{");

        for field in &expression.fields {
            self.emit_str(".");
            self.emit_token(field.identifier.as_str());
            self.emit_token("=");
            self.emit_expression(&field.expression);
            self.emit_token(",")
        }

        self.emit_str("}");
    }
}
