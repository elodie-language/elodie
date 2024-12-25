use std::collections::HashMap;
use crate::common::{Symbol, SymbolId, SymbolName};


struct Frame {
    variables: HashMap<SymbolName, SymbolId>,
}

impl Frame {
    fn new() -> Self {
        Self {
            variables: HashMap::new()
        }
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

    pub(crate) fn register_symbol(&mut self, symbol: &Symbol) {
        let frame = self.frames.last_mut().unwrap();
        match symbol {
            Symbol::Variable(s) => {
                frame.variables.insert(s.name.clone(), s.id.clone());
            }
            _ => unimplemented!()
        }
    }

    pub(crate) fn enter(&mut self) {
        self.frames.push(Frame::new())
    }

    pub(crate) fn leave(&mut self) {
        self.frames.pop().unwrap();
    }
}