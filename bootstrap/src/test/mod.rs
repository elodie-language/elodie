use std::{fs, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::rc::Rc;

use crate::{load_library_file, load_test_runner};
use crate::common::Context;
use crate::compile::compile_str;
use crate::run::run;
use crate::run::scope::Scope;
use crate::run::type_definitions::TypeDefinitions;
use crate::run::value::{IntrinsicFunctionValue, ObjectValue, Value};
use crate::run::value::Value::IntrinsicFunction;

pub fn test_files(files: Vec<PathBuf>, print_colors: bool) {
    let _ = fs::remove_dir("/tmp/elodie");
    fs::create_dir("/tmp/elodie").expect("Failed to create test directory");

    test_file(files.first().unwrap(), print_colors);
}

fn test_file(file: &PathBuf, print_colors: bool) {
    let mut ctx = Context::new();
    let mut root_values = HashMap::new();
    let mut root_types = HashMap::new();


    let print_colors = print_colors.clone();
    let mut intrinsics = ObjectValue::new();
    intrinsics.set_property(
        ctx.string_cache.insert("print"),
        IntrinsicFunction(IntrinsicFunctionValue(Rc::new(move |args: &[Value]|{
            for arg in args {
                if arg.to_string() == "\\n" {
                    println!();
                } else {
                    if print_colors {
                        print!("{} ", arg.to_string().replace("\\x1b","\x1b"));
                    } else {
                        print!("{} ", arg.to_string());
                    }
                }
            }
            Ok(Value::Unit)
        }))),
    );

    intrinsics.set_property(
        ctx.string_cache.insert("list_length"),
        IntrinsicFunction(IntrinsicFunctionValue(Rc::new(|args| {
            let Value::List(list) = args.get(0).unwrap() else { panic!("not list") };
            let len: u32 = list.0.borrow().len() as u32;
            Ok(Value::Number(len.into()))
        }))),
    );

    intrinsics.set_property(
        ctx.string_cache.insert("list_append"),
        IntrinsicFunction(IntrinsicFunctionValue(Rc::new(|args| {
            let Value::List(list) = args.get(0).unwrap() else { panic!("not list") };
            let arg = args.get(1).cloned().unwrap();
            list.0.borrow_mut().push(arg);
            Ok(Value::Unit)
        }))),
    );

    intrinsics.set_property(
        ctx.string_cache.insert("list_get"),
        IntrinsicFunction(IntrinsicFunctionValue(Rc::new(|args| {
            let Value::List(list) = args.get(0).unwrap() else { panic!("not list") };
            let Value::Number(arg) = args.get(1).cloned().unwrap() else { panic!("not a number") };
            Ok(list.0.borrow().get(arg as usize - 1).cloned().unwrap())
        }))),
    );

    root_values.insert(ctx.string_cache.insert("intrinsics"), Value::Object(intrinsics));
    let scope = Scope::new(
        root_values,
        root_types,
    );

// load std
    let (scope, definitions) = {
        let std_content = load_library_file("std/index.ec").unwrap();
        let std_file = compile_str(&mut ctx, std_content.as_str()).unwrap();
        run(&mut ctx, scope, TypeDefinitions { definitions: Default::default() }, std_file).unwrap()
    };

// load test runner
    let (scope, definitions) = {
        let std_content = load_test_runner().unwrap();
        let std_file = compile_str(&mut ctx, std_content.as_str()).unwrap();
        run(&mut ctx, scope, definitions, std_file).unwrap()
    };

    let mut path = PathBuf::from(file);
    let content = load_text_from_file(path.to_str().unwrap()).unwrap();
    let source_file = compile_str(&mut ctx, content.as_str()).unwrap();

    run(&mut ctx, scope, definitions, source_file).unwrap();
}

fn load_text_from_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
