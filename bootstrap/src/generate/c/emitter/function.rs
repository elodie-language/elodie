use crate::generate::c;
use crate::generate::c::emitter::Emitter;

impl Emitter {
    pub(crate) fn emit_call_function(&mut self, node: &c::CallFunctionStatement) {
        if let Some(result) = &node.result {
            self.emit_token(result.r#type.as_str());
            self.emit_token(result.identifier.as_str());
            self.emit_token("=");
        }

        self.emit_str(node.identifier.as_str());
        self.emit_str("(");

        for (idx, arg) in node.arguments.iter().enumerate() {
            if idx > 0 {
                self.emit_token(",");
            }
            self.emit_expression(arg)
        }
        self.emit_line(");");
    }

    pub(crate) fn emit_declare_function(&mut self, node: &c::DeclareFunctionNode) {
        todo!()
    }

    pub(crate) fn emit_define_function(&mut self, node: &c::DefineFunctionNode) {
        self.emit_token(&node.ty);
        self.emit_str(&node.identifier);
        self.emit_token("(void)");

        self.emit_block_statement(&node.statements);
    }

    pub(crate) fn emit_return_from_function(&mut self, node: &c::ReturnFromFunctionStatement) {
        self.emit_line("return 0;")
    }
}