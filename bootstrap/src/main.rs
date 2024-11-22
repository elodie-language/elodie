use std::{env, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::rc::Rc;

use crate::common::Context;
use crate::compile::compile_str;
use crate::run::run;
use crate::run::scope::Scope;
use crate::run::value::{HostFunctionValue, ObjectValue, Value};
use crate::run::value::Value::HostFunction;

mod common;
mod cli;
mod ast;
mod compile;
mod run;
mod lex;
mod parse;
mod r#type;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut ctx = Context::new();


    let mut root_values = HashMap::new();
    let mut root_types = HashMap::new();

    let mut intrinsics = ObjectValue::new();
    intrinsics.set_property(
        ctx.string_cache.insert("print"),
        HostFunction(HostFunctionValue(Rc::new(|args: &[Value]| {
            for arg in args {
                if arg.to_string() == "\\n" {
                    println!();
                } else {
                    print!("{} ", arg.to_string());
                }
            }
            Ok(Value::Unit)
        }))),
    );

    root_values.insert(ctx.string_cache.insert("intrinsics"), Value::Object(intrinsics));
    let scope = Scope::new(
        root_values,
        root_types,
    );


    let scope = {
        let std_content = load_library_file("std/index.elx").unwrap();
        let std_file = compile_str(&mut ctx, std_content.as_str()).unwrap();
        run(&mut ctx, scope, std_file).unwrap()
    };


    let mut path = PathBuf::from(args.get(1).unwrap());
    let content = load_text_from_file(path.to_str().unwrap()).unwrap();
    let source_file = compile_str(&mut ctx, content.as_str()).unwrap();

    run(&mut ctx, scope, source_file).unwrap();
}

fn load_text_from_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn load_library_file(filename: &str) -> io::Result<String> {
    let manifest_dir = "/home/ddymke/repo/elodie/src/lib/";
    let file_path = PathBuf::from(manifest_dir).join(filename);

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
