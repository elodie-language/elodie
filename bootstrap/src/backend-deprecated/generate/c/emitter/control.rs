use crate::backend::generate::c::emitter::Emitter;
use crate::backend::generate::c::IfStatement;

impl Emitter {
    pub(crate) fn if(&mut self, statement: &IfStatement) {
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
