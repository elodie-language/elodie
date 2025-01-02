use crate::build::c;
use crate::build::c::emitter::Emitter;

impl Emitter {

    pub(crate) fn call_function(&mut self, node: &c::CallFunctionStatement) {
        if let Some(result) = &node.result {
            self.token(result.r#type.as_str());
            self.token(result.identifier.as_str());
            self.token("=");
        }

        self.str(node.function.as_str());
        self.str("(");

        for (idx, arg) in node.arguments.iter().enumerate() {
            if idx > 0 {
                self.token(",");
            }
            self.expression(arg)
        }
        self.line(");");
    }

    pub(crate) fn call_function_expression(&mut self, node: &c::CallFunctionExpression) {
        self.str(node.function.as_str());
        self.str("(");

        for (idx, arg) in node.arguments.iter().enumerate() {
            if idx > 0 {
                self.token(",");
            }
            self.expression(arg)
        }
        self.str(")");
    }

    pub(crate) fn declare_function(&mut self, node: &c::DeclareFunctionNode) {
        self.token(&node.ty);
        self.str(&node.identifier);

        if node.arguments.is_empty() {
            self.str("(void)");
        } else {
            self.str("(");
            for (idx, arg) in node.arguments.iter().enumerate() {
                self.token(arg.ty.as_str());
                self.str(arg.identifier.as_str());
                if idx < node.arguments.len() - 1 {
                    self.token(",")
                }
            }
            self.str(")");
        }

        self.line(";");
    }

    pub(crate) fn define_function(&mut self, node: &c::DefineFunctionNode) {
        self.token(&node.ty);
        self.str(&node.identifier);

        if node.arguments.is_empty() {
            self.str("(void)");
        } else {
            self.str("(");
            for (idx, arg) in node.arguments.iter().enumerate() {
                self.token(arg.ty.as_str());
                self.str(arg.identifier.as_str());
                if idx < node.arguments.len() - 1 {
                    self.token(",")
                }
            }
            self.str(")");
        }

        self.block_statement(&node.statements);
    }

    pub(crate) fn return_from_function(&mut self, node: &c::ReturnFromFunctionStatement) {
        self.token("return");
        if let Some(expression) = &node.node {
            self.expression(expression);
        }
        self.line(";")
    }
}
