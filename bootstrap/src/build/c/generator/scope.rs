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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Storage {
    Memory,
    Stack,
}

pub enum Variable {
    Argument(Argument, Storage),
    Variable(LocalVariable, Storage),
    Temp(Temp, Storage),
}

impl Variable {
    pub fn to_string(&self) -> String {
        match self {
            Variable::Argument(a, ..) => a.to_string(),
            Variable::Variable(v, ..) => v.to_string(),
            Variable::Temp(t, ..) => t.to_string()
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
    args_storage: Vec<Storage>,
    local_variables: Vec<LocalVariable>,
    local_variables_storage: Vec<Storage>,
    temps: u16,
    temps_storage: Vec<Storage>,
    pub statements: Vec<Statement>,
}

impl Frame {
    fn new() -> Self {
        Self {
            args: 0,
            args_storage: vec![],
            local_variables: vec![],
            local_variables_storage: vec![],
            temps: 0,
            temps_storage: vec![],
            statements: vec![],
        }
    }

    fn push_argument(&mut self, storage: Storage) -> Argument {
        self.args += 1;
        self.args_storage.push(storage);
        Argument(self.args)
    }

    fn push_local_variable(&mut self, variable: String, storage: Storage) -> LocalVariable {
        let result = LocalVariable(variable);
        self.local_variables.push(result.clone());
        self.local_variables_storage.push(storage);
        result
    }

    fn push_temp(&mut self, storage: Storage) -> Temp {
        self.temps += 1;
        self.temps_storage.push(storage);
        Temp(self.temps)
    }

    pub fn cleanup(&mut self) -> Vec<c::Statement> {
        let mut result = vec![];

        for arg in 0..self.args {
            if self.args_storage[arg as usize] == Storage::Memory {
                result.push(Statement::rc_dec(Argument(arg + 1)))
            }
        }

        let mut counter = 0;
        while let Some(local) = self.local_variables.pop() {
            if self.local_variables_storage[counter] == Storage::Memory {
                result.push(Statement::rc_dec(local))
            }
            counter += 1
        }

        for temp in 0..self.temps {
            if self.temps_storage[temp as usize] == Storage::Memory {
                result.push(Statement::rc_dec(Temp(temp + 1)))
            }
        }

        result
    }
}

pub(crate) struct Scope {
    frames: Vec<Frame>,
}

impl Scope {
    pub(crate) fn new() -> Self {
        Self {
            frames: vec![Frame::new()]
        }
    }

    pub(crate) fn enter(&mut self) {
        self.frames.push(Frame::new())
    }

    // pub(crate) fn leave(&mut self) {
    //     let frame = self.frames.pop().unwrap();
    //     // frame.cleanup()
    //     self.statements().push(Statement::Block(BlockStatement{ statements: frame.statements}));
    // }

    pub(crate) fn leave(&mut self) -> Frame {
        self.frames.pop().unwrap()
    }

    pub(crate) fn push_argument(&mut self, storage: Storage) -> Argument {
        self.frames.last_mut().unwrap().push_argument(storage)
    }

    pub(crate) fn push_local_variable(&mut self, variable: String, storage: Storage) -> LocalVariable {
        self.frames.last_mut().unwrap().push_local_variable(variable, storage)
    }

    pub(crate) fn push_temp(&mut self, storage: Storage) -> Temp {
        self.frames.last_mut().unwrap().push_temp(storage)
    }

    pub(crate) fn statements(&mut self) -> &mut Vec<Statement> {
        &mut self.frames.last_mut().unwrap().statements
    }
}