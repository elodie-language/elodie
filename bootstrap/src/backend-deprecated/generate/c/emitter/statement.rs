use crate::backend::generate::c;
use crate::backend::generate::c::emitter::Emitter;
use crate::backend::generate::c::Statement;

impl Emitter {
    pub(crate) fn statement(&mut self, statement: &c::Statement) {
        match statement {
            Statement::Block(statement) => self.block_statement(statement),
            Statement::CallFunction(statement) => self.call_function(statement),
            Statement::DeclareArray(statement) => self.declare_array(statement),
            Statement::DeclareVariable(statement) => self.declare_variable(statement),
            Statement::If(statement) => self.if(statement),
            Statement::ReturnFromFunction(statement) => self.return_from_function(statement),
        }
    }

    pub(crate) fn block_statement(&mut self, statement: &c::BlockStatement) {
        self.line("{");
        for statement in &statement.statements {
            self.statement(statement);
        }
        self.line("}");
    }
}
