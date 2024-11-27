use crate::generate::c;
use crate::generate::c::emitter;
use crate::generate::c::emitter::Emitter;

impl Emitter {
    pub(crate) fn emit_call_function(&mut self, node: &c::CallFunctionExpression) -> emitter::Result<()> {
        todo!()
    }

    pub(crate) fn emit_declare_function(&mut self, node: &c::DeclareFunctionNode) -> emitter::Result<()> {
        todo!()
    }

    pub(crate) fn emit_define_function(&mut self, node: &c::DefineFunctionNode) -> emitter::Result<()> {
        self.emit_token(&node.ty);
        self.emit_str(&node.identifier);
        self.emit_token("(void)");

        self.emit_line("{");
        self.emit_line("printf(\"Elodie say's hi\\n\");");
        self.emit_line("return 0;");
        self.emit_line("}");

        Ok(())
    }

    pub(crate) fn emit_return_from_function(&mut self, node: &c::ReturnFromFunctionStatement) -> emitter::Result<()> {
        todo!()
    }
}