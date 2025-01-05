use crate::build::c;
use crate::build::c::{CallFunctionStatement, Statement, VariableExpression};

impl Statement {
    pub fn rc_dec<T: Into<String>>(variable: T) -> c::Statement {
        Statement::CallFunction(CallFunctionStatement {
            function: "val_rc_dec".to_string(),
            arguments: Box::new([
                c::Expression::Variable(VariableExpression { variable: variable.into(), cast: None })
            ]),
            result: None,
        })
    }
}