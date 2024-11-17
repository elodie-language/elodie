use crate::ast::{Ast, SourceFile};
use crate::runner::scope::Scope;
use crate::runner::value::Value;

mod scope;
mod value;

#[derive(Debug)]
pub enum Error {}

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub struct Runner {
    scope: Scope,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            scope: Scope::new()
        }
    }

    pub fn run(&mut self, source_file: SourceFile) -> Result<()> {
        for node in source_file.body {
            match node {
                Ast::CallFunctionOfObject { object, function, arguments } => {
                    let Value::Object(object) = self.scope.get(object.as_str()).unwrap() else { panic!() };
                    let Value::HostFunction(func) = object.get_property(function.as_str()).unwrap() else { panic!() };

                    let Ast::StringValue(arg_1) = arguments.first().unwrap() else { panic!() };

                    func.0(&[&Value::String(arg_1.to_string())]).unwrap();
                }
                _ => todo!()
            }
        }

        // println!("{:?}", source_file.body);
        Ok(())
    }
}