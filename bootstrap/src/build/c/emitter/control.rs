use crate::build::c::{BreakStatement, IfStatement, LoopStatement, StatementResult};
use crate::build::c::emitter::Emitter;

impl Emitter {
    pub(crate) fn r#break(&mut self, statement: &BreakStatement) {
        self.line("break;")
    }

    pub(crate) fn r#loop(&mut self, statement: &LoopStatement) {
        if let Some(result) = &statement.result {
            match result {
                StatementResult::Assign { variable } => {
                    self.token(variable.as_str())
                }
                StatementResult::Declare { variable, r#type } => {
                    self.token(r#type.as_str());
                    self.token(variable.as_str());
                }
            }

            self.token("=");
            self.line("nullptr;");
        }
        self.token("while (true)");
        self.block_statement(&statement.block)
    }

    pub(crate) fn r#if(&mut self, statement: &IfStatement) {
        self.token("if");
        self.str("(");
        self.expression(&statement.condition);
        self.token(")");
        self.block_statement(&statement.then);
        if let Some(otherwise) = &statement.otherwise {
            self.token("else");
            self.block_statement(otherwise);
        }
    }
}
