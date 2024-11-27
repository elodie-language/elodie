use crate::generate::c;
use crate::generate::c::{Indent, Node};

mod directive;
mod function;

pub enum Error {}

pub(crate) type Result<T> = core::result::Result<T, Error>;

pub(crate) fn emit(nodes: &[c::Node]) -> Result<String> {
    let mut emitter = Emitter {
        output: String::new(),
        indent: Indent::none(),
    };
    emitter.emit(nodes)
}

pub(crate) struct Emitter {
    pub(crate) output: String,
    pub(crate) indent: Indent,
}

impl Emitter {
    pub(crate) fn emit(mut self, nodes: &[c::Node]) -> Result<String> {
        for node in nodes {
            match node {
                Node::Directive(node) => self.emit_directive(node)?,
                Node::DeclareFunction(node) => self.emit_declare_function(node)?,
                Node::DeclareStruct(_) => unimplemented!(),
                Node::DefineFunction(node) => self.emit_define_function(node)?,
                Node::DefineStruct(_) => unimplemented!(),
                Node::DefineGlobalVariable(_) => unimplemented!(),
            }
        }

        Ok(self.output)
    }

    pub(crate) fn emit_str(&mut self, str: &str) {
        self.output.push_str(str);
    }

    pub(crate) fn emit_token(&mut self, token: &str) {
        self.output.push_str(token);
        self.output.push_str(" ");
    }

    pub(crate) fn emit_line(&mut self, line: &str) {
        self.output.push_str(line);
        self.output.push_str("\n");
    }
}