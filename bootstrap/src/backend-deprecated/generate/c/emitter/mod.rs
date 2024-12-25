use crate::backend::generate::c;
use crate::backend::generate::c::{Expression, Indent, Node};

mod control;
mod directive;
mod function;
mod infix;
mod literal;
mod statement;
mod r#struct;
mod variable;

pub enum Error {}

pub(crate) type Result<T> = core::result::Result<T, Error>;

pub(crate) fn emit(nodes: &[c::Node]) -> String {
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
    pub(crate) fn emit(mut self, nodes: &[c::Node]) -> String {
        for node in nodes {
            match node {
                Node::Directive(node) => self.emit_directive(node),
                Node::DeclareFunction(node) => self.emit_declare_function(node),
                Node::DeclareStruct(node) => self.emit_declare_struct(node),
                Node::DefineFunction(node) => self.emit_define_function(node),
                Node::DefineStruct(node) => self.emit_define_struct(node),
                Node::DefineGlobalVariable(_) => unimplemented!(),
            }
        }
        self.output
    }

    pub(crate) fn emit_expression(&mut self, expression: &c::Expression) {
        match expression {
            Expression::Literal(expression) => self.emit_literal(expression),
            Expression::Infix(expression) => self.emit_infix(expression),
            Expression::Variable(expression) => self.emit_variable(expression),
            Expression::StructInitialisation(expression) => self.emit_initialise_struct(expression),
        }
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
