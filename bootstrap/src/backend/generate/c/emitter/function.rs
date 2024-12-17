use crate::backend::generate::c;
use crate::backend::generate::c::emitter::Emitter;

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
        self.emit_token(&node.ty);
        self.emit_str(&node.identifier);

        if node.arguments.is_empty() {
            self.emit_str("(void)");
        } else {
            self.emit_str("(");
            for (idx, arg) in node.arguments.iter().enumerate() {
                self.emit_token(arg.ty.as_str());
                self.emit_str(arg.identifier.as_str());
                if idx < node.arguments.len() - 1 {
                    self.emit_token(",")
                }
            }
            self.emit_str(")");
        }


        self.emit_line(";");
    }

    pub(crate) fn emit_define_function(&mut self, node: &c::DefineFunctionNode) {
        self.emit_token(&node.ty);
        self.emit_str(&node.identifier);

        if node.arguments.is_empty() {
            self.emit_str("(void)");
        } else {
            self.emit_str("(");
            for (idx, arg) in node.arguments.iter().enumerate() {
                self.emit_token(arg.ty.as_str());
                self.emit_str(arg.identifier.as_str());
                if idx < node.arguments.len() - 1 {
                    self.emit_token(",")
                }
            }
            self.emit_str(")");
        }

        self.emit_block_statement(&node.statements);
    }

    pub(crate) fn emit_return_from_function(&mut self, node: &c::ReturnFromFunctionStatement) {
        self.emit_token("return");
        if let Some(expression) = &node.node {
            self.emit_expression(expression);
        }
        self.emit_line(";")
    }
}