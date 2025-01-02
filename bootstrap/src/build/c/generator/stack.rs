use crate::build::c;
use crate::build::c::Statement;

#[derive(Clone, Copy, Debug)]
pub struct Argument(pub u16);

impl Argument {
    pub fn to_string(&self) -> String {
        format!("arg_{}", self.0)
    }
}

impl Into<String> for Argument {
    fn into(self) -> String {
        self.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct LocalVariable(pub String);

impl LocalVariable {
    pub fn to_string(&self) -> String { self.0.clone() }
}

impl Into<String> for LocalVariable {
    fn into(self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Temp(pub u16);

impl Temp {
    pub fn to_string(&self) -> String {
        format!("temp_{}", self.0)
    }
}

impl Into<String> for Temp {
    fn into(self) -> String {
        self.to_string()
    }
}

pub enum Variable {
    Argument(Argument),
    Variable(LocalVariable),
    Temp(Temp),
}

impl Variable {
    pub fn to_string(&self) -> String {
        match self {
            Variable::Argument(a) => a.to_string(),
            Variable::Variable(v) => v.to_string(),
            Variable::Temp(t) => t.to_string()
        }
    }
}

impl Into<String> for Variable {
    fn into(self) -> String {
        self.to_string()
    }
}

pub(crate) struct Frame {
    args: u16,
    local_variables: Vec<LocalVariable>,
    temps: u16,
}

impl Frame {
    fn new() -> Self {
        Self {
            args: 0,
            local_variables: vec![],
            temps: 0,
        }
    }

    fn push_argument(&mut self) -> Argument {
        self.args += 1;
        Argument(self.args)
    }

    fn push_local_variable(&mut self, variable: String) -> LocalVariable {
        let result = LocalVariable(variable);
        self.local_variables.push(result.clone());
        result
    }

    fn push_temp(&mut self) -> Temp {
        self.temps += 1;
        Temp(self.temps)
    }

    fn cleanup(&mut self) -> Vec<c::Statement> {
        let mut result = vec![];

        for arg in 0..self.args {
            result.push(Statement::rc_dec(Argument(arg + 1)))
        }

        while let Some(local) = self.local_variables.pop() {
            result.push(Statement::rc_dec(local))
        }

        for temp in 0..self.temps {
            result.push(Statement::rc_dec(Temp(temp + 1)))
        }

        result
    }
}

pub(crate) struct Stack {
    frames: Vec<Frame>,
}

impl Stack {
    pub(crate) fn new() -> Self {
        Self {
            frames: vec![Frame::new()]
        }
    }

    pub(crate) fn enter(&mut self) {
        self.frames.push(Frame::new())
    }

    pub(crate) fn leave(&mut self) -> Vec<Statement> {
        let mut frame = self.frames.pop().unwrap();
        frame.cleanup()
    }

    pub(crate) fn push_argument(&mut self) -> Argument {
        self.frames.last_mut().unwrap().push_argument()
    }

    pub(crate) fn push_local_variable(&mut self, variable: String) -> LocalVariable {
        self.frames.last_mut().unwrap().push_local_variable(variable)
    }

    pub(crate) fn push_temp(&mut self) -> Temp {
        self.frames.last_mut().unwrap().push_temp()
    }
}