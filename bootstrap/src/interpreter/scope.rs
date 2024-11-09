use std::{fs, vec};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;

use crate::interpreter::value::{BuiltinFunction, Object, Value};

#[derive(Clone)]
pub enum LoopInterrupt {
    Break(Value),
    Continue,
    Return(Value),
}

pub struct Scope {
    pub values: Vec<HashMap<String, Value>>,
    pub loop_interrupt: Option<LoopInterrupt>,
}

impl Scope {
    pub fn new() -> Self {
        let mut result = Self {
            values: vec![],
            loop_interrupt: None,
        };

        let mut root = HashMap::new();

        let mut console = Object::new();
        console.set_property(
            "log",
            Value::BuiltinFunction(BuiltinFunction(Rc::new(|args: &[Value]| {
                for arg in args {
                    print!("{} ", arg.to_string());
                }
                println!();
                Ok(Value::Unit)
            }))),
        );

        root.insert("console".to_string(), Value::Object(console));


        let mut fs = Object::new();

        fs.set_property(
            "create_directory",
            Value::BuiltinFunction(BuiltinFunction(Rc::new(|args: &[Value]| {
                if let Some(Value::String(dir)) = args.get(0) {
                    match fs::create_dir_all(Path::new(dir)) {
                        Ok(_) => Ok(Value::Unit),
                        Err(e) => {
                            eprintln!("Error creating directory: {}", e);
                            Ok(Value::Unit)
                        }
                    }
                } else {
                    eprintln!("create_directory expects a string argument");
                    Ok(Value::Unit)
                }
            }))),
        );

        fs.set_property(
            "create_file",
            Value::BuiltinFunction(BuiltinFunction(Rc::new(|args: &[Value]| {
                if let Some(Value::String(file_path)) = args.get(0) {
                    match fs::File::create(Path::new(file_path)) {
                        Ok(_) => Ok(Value::Unit),
                        Err(e) => {
                            eprintln!("Error creating file: {}", e);
                            Ok(Value::Unit)
                        }
                    }
                } else {
                    eprintln!("create_file expects a string argument");
                    Ok(Value::Unit)
                }
            }))),
        );

        fs.set_property(
            "write_to_file",
            Value::BuiltinFunction(BuiltinFunction(Rc::new(|args: &[Value]| {
                if let (Some(Value::String(file_path)), Some(Value::String(content))) =
                    (args.get(0), args.get(1))
                {
                    match fs::OpenOptions::new().write(true).append(true).open(file_path) {
                        Ok(mut file) => {
                            if let Err(e) = file.write_all(content.as_bytes()) {
                                eprintln!("Error writing to file: {}", e);
                            }

                            file.write("\n".as_bytes()).unwrap();
                        }
                        Err(e) => {
                            eprintln!("Error opening file: {}", e);
                        }
                    }
                    Ok(Value::Unit)
                } else {
                    eprintln!("write_to_file expects two string arguments");
                    Ok(Value::Unit)
                }
            }))),
        );

        root.insert("fs".to_string(), Value::Object(fs));

        result.values.push(root);

        result
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        for scope in self.values.iter().rev() {
            if let Some(value) = scope.get(key) {
                return Some(value);
            }
        }
        None
    }

    pub fn insert(&mut self, name: impl Into<String>, value: Value) {
        self.values.last_mut().unwrap().insert(name.into(), value);
    }

    pub fn enter(&mut self) {
        self.values.push(HashMap::new());
    }

    pub fn leave(&mut self) {
        self.values.pop().unwrap();
    }

    pub fn interrupt_loop(&mut self, loop_interrupt: LoopInterrupt) {
        self.loop_interrupt = Some(loop_interrupt)
    }

    pub fn reset_loop_interrupt(&mut self) {
        self.loop_interrupt = None
    }
}

