use crate::build::c::emitter::Emitter;
use crate::build::c::IfStatement;

impl Emitter {
    pub(crate) fn r#if(&mut self, statement: &IfStatement) {
        self.token("if");
        self.token("(");
        self.expression(&statement.condition);
        self.token(")");
        self.block_statement(&statement.then);
        if let Some(otherwise) = &statement.otherwise {
            self.token("else");
            self.block_statement(otherwise);
        }
    }
}
