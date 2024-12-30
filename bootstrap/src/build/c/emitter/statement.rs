use crate::build::c;
use crate::build::c::emitter::Emitter;
use crate::build::c::Statement;

impl Emitter {
    pub(crate) fn statement(&mut self, statement: &c::Statement) {
        match statement {
            Statement::Block(statement) => self.block_statement(statement),
            Statement::CallFunction(statement) => self.call_function(statement),
            Statement::Code(statement) => self.code_statement(statement),
            Statement::DeclareArray(statement) => self.declare_array(statement),
            Statement::DeclareVariable(statement) => self.declare_variable(statement),
            Statement::If(statement) => self.r#if(statement),
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
