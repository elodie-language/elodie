use std::{fs, vec};
use std::collections::HashMap;
use std::io::Write;
use std::ops::Index;
use std::path::Path;
use std::rc::Rc;

use crate::interpreter::value::{Function, Object, Value};

pub struct Scope {
    values: Vec<HashMap<String, Value>>,
}

impl Scope {

    pub fn new() -> Self {
        let mut result = Self {
            values: vec![],
        };

        let mut root = HashMap::new();

        let mut console = Object::new();
        console.set_property(
            "log",
            Value::Function(Function(Rc::new(|args: &[Value]| {
                for arg in args {
                    print!("{} ", arg.to_string());
                }
                println!();
                Value::Unit
            }))),
        );

        root.insert("console".to_string(), Value::Object(console));


        let mut fs = Object::new();

        fs.set_property(
            "create_directory",
            Value::Function(Function(Rc::new(|args: &[Value]| {
                if let Some(Value::String(dir)) = args.get(0) {
                    match fs::create_dir_all(Path::new(dir)) {
                        Ok(_) => Value::Unit,
                        Err(e) => {
                            eprintln!("Error creating directory: {}", e);
                            Value::Unit
                        }
                    }
                } else {
                    eprintln!("create_directory expects a string argument");
                    Value::Unit
                }
            }))),
        );

        fs.set_property(
            "create_file",
            Value::Function(Function(Rc::new(|args: &[Value]| {
                if let Some(Value::String(file_path)) = args.get(0) {
                    match fs::File::create(Path::new(file_path)) {
                        Ok(_) => Value::Unit,
                        Err(e) => {
                            eprintln!("Error creating file: {}", e);
                            Value::Unit
                        }
                    }
                } else {
                    eprintln!("create_file expects a string argument");
                    Value::Unit
                }
            }))),
        );

        fs.set_property(
            "write_to_file",
            Value::Function(Function(Rc::new(|args: &[Value]| {
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
                    Value::Unit
                } else {
                    eprintln!("write_to_file expects two string arguments");
                    Value::Unit
                }
            }))),
        );

        root.insert("fs".to_string(), Value::Object(fs));

        result.values.push(root);

        result
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.last()?.get(name)
    }

    pub fn insert(&mut self, name: impl Into<String>, value: Value){
        self.values.last_mut().unwrap().insert(name.into(), value);
    }
}
