use crate::backend::build::c;
use crate::backend::build::c::emitter::Emitter;
use crate::backend::build::c::Statement;

impl Emitter {
    pub(crate) fn emit_statement(&mut self, statement: &c::Statement) {
        match statement {
            Statement::Block(statement) => self.emit_block_statement(statement),
            Statement::CallFunction(statement) => self.emit_call_function(statement),
            Statement::DeclareArray(statement) => self.emit_declare_array(statement),
            Statement::DeclareVariable(statement) => self.emit_declare_variable(statement),
            Statement::If(statement) => self.emit_if(statement),
            Statement::ReturnFromFunction(statement) => self.emit_return_from_function(statement),
        }
    }

    pub(crate) fn emit_block_statement(&mut self, statement: &c::BlockStatement) {
        self.emit_line("{");
        for statement in &statement.statements {
            self.emit_statement(statement);
        }
        self.emit_line("}");
    }
}
