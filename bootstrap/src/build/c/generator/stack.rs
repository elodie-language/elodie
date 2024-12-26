use std::collections::HashSet;

use crate::build::c;
use crate::build::c::{DeclareFunctionNode, DeclareStructNode, DirectiveNode};

struct Function {

    statements: Vec<c::Statement>,
}

impl Function {
    pub(crate) fn new() -> Self { Self { statements: vec![] } }
}

struct Frame {
    function: Function,
    directives: HashSet<DirectiveNode>,
    function_declarations: Vec<DeclareFunctionNode>,
    struct_declarations: Vec<DeclareStructNode>,
}

impl Frame {
    pub(crate) fn new() -> Self {
        Self {
            function: Function::new(),
            directives: HashSet::new(),
            function_declarations: Vec::new(),
            struct_declarations: Vec::new(),
        }
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

    pub(crate) fn leave(&mut self) {
        self.frames.pop().unwrap();
    }

    pub(crate) fn push_statement(&mut self, stmt: c::Statement) {
        self.frames.last_mut().unwrap().function.statements.push(stmt);
    }
}