use crate::generate::c;
use crate::generate::c::emitter::Emitter;

impl Emitter {

    pub(crate) fn emit_call_function(&mut self, node: &c::CallFunctionExpression)  {
        self.emit_str(node.identifier.as_str());
        self.emit_str("(\"Elodie say's hi\")");
    }

    pub(crate) fn emit_declare_function(&mut self, node: &c::DeclareFunctionNode)  {
        todo!()
    }

    pub(crate) fn emit_define_function(&mut self, node: &c::DefineFunctionNode)  {
        dbg!(node);

        self.emit_token(&node.ty);
        self.emit_str(&node.identifier);
        self.emit_token("(void)");

        self.emit_block_statement(&node.statements);
    }

    pub(crate) fn emit_return_from_function(&mut self, node: &c::ReturnFromFunctionStatement)  {
        self.emit_line("return 0;")
    }
}