use crate::build::c::emitter::Emitter;
use crate::build::c::{
    DeclareStructNode, DefineStructNode, InitialiseStructExpression, InitialiseStructField,
};

impl Emitter {
    pub(crate) fn declare_struct(&mut self, node: &DeclareStructNode) {
        self.token("struct");
        self.str(node.identifier.as_str());
        self.line(";");
    }

    pub(crate) fn define_struct(&mut self, node: &DefineStructNode) {
        self.token("struct");
        self.str(node.identifier.as_str());
        self.line("{");

        for field in &node.fields {
            self.token(field.ty.as_str());
            self.str(field.identifier.as_str());
            self.line(";");
        }

        self.line("};");
    }

    pub(crate) fn initialise_struct(&mut self, expression: &InitialiseStructExpression) {
        self.str("{");

        for field in &expression.fields {
            self.str(".");
            self.token(field.identifier.as_str());
            self.token("=");
            self.expression(&field.expression);
            self.token(",")
        }

        self.str("}");
    }
}
