use std::collections::HashMap;

use crate::common::{Span, Symbol, SymbolId, SymbolName};

struct Frame {
    span: Span,
    variables: HashMap<SymbolName, SymbolId>,
}

impl Frame {
    fn new() -> Self {
        Self {
            span: Span::default(),
            variables: HashMap::new(),
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

    pub(crate) fn span_set(&mut self, span: Span) {
        self.frames.last_mut().unwrap().span = span
    }

    pub(crate) fn span_get(&self) -> Span {
        self.frames.last().unwrap().span.clone()
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

    pub(crate) fn variable(&self, name: impl AsRef<SymbolName>) -> Option<SymbolId> {
        for frame in self.frames.iter().rev() {
            if let Some(value) = frame.variables.get(name.as_ref()).cloned() {
                return Some(value);
            }
        }
        None
    }

    pub(crate) fn enter(&mut self) {
        self.frames.push(Frame::new())
    }

    pub(crate) fn leave(&mut self) {
        self.frames.pop().unwrap();
    }
}