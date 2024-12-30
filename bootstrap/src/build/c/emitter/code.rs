use crate::build::c;
use crate::build::c::emitter::Emitter;

impl Emitter {
    pub(crate) fn code_node(&mut self, node: &c::CodeNode) {
        self.line(node.code.as_str())
    }

    pub(crate) fn code_statement(&mut self, statement: &c::CodeStatement) {
        self.line(statement.code.as_str())
    }
}