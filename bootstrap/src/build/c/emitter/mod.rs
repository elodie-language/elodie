use crate::build::c;
use crate::build::c::{Expression, Node};

mod control;
mod directive;
mod function;
mod infix;
mod literal;
mod statement;
mod r#struct;
mod variable;
mod code;
mod operator;

pub enum Error {}

pub(crate) type Result<T> = core::result::Result<T, Error>;

pub(crate) fn emit(nodes: &[c::Node]) -> String {
    let mut emitter = Emitter {
        output: String::new(),
    };
    emitter.emit(nodes)
}

pub(crate) struct Emitter {
    pub(crate) output: String,
}

impl Emitter {
    pub(crate) fn emit(mut self, nodes: &[c::Node]) -> String {
        for node in nodes {
            match node {
                Node::Code(node) => self.code_node(node),
                Node::Directive(node) => self.directive(node),
                Node::DeclareFunction(node) => self.declare_function(node),
                Node::DeclareStruct(node) => self.declare_struct(node),
                Node::DefineFunction(node) => self.define_function(node),
                Node::DefineStruct(node) => self.define_struct(node),
                Node::DefineGlobalVariable(_) => unimplemented!(),
            }
        }
        self.output
    }

    pub(crate) fn expression(&mut self, expression: &c::Expression) {
        match expression {
            Expression::AccessVariableOfStruct(expression) => self.access_variable_of_object(expression),
            Expression::CallFunction(expression) => self.call_function_expression(expression),
            Expression::Code(expression) => self.code_expression(expression),
            Expression::Compare(expression) => self.compare(expression),
            Expression::Infix(expression) => self.infix(expression),
            Expression::Literal(expression) => self.literal(expression),
            Expression::Variable(expression) => self.variable(expression),
            Expression::StructInitialisation(expression) => self.initialise_struct(expression),
        }
    }

    pub(crate) fn str(&mut self, str: &str) {
        self.output.push_str(str);
    }

    pub(crate) fn token(&mut self, token: &str) {
        self.output.push_str(token);
        self.output.push_str(" ");
    }

    pub(crate) fn line(&mut self, line: &str) {
        self.output.push_str(line);
        self.output.push_str("\n");
    }
}
