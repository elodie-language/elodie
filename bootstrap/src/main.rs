use std::{env, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;

use crate::common::Context;
use crate::compile::compile_str;
use crate::run::{run, run_file};
use crate::run::scope::Scope;
use crate::run::type_definitions::TypeDefinitions;
use crate::test::test_files;

mod common;
mod cli;
mod ir;
mod compile;
mod run;
mod lex;
mod parse;
mod r#type;
mod test;
mod generate;
mod build;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        eprintln!("Requires at least one argument");
        exit(1)
    }

    if args.get(1).unwrap() == "build" {
        let file = PathBuf::from(args.get(2).unwrap());

        fn load_text_from_file(path: &str) -> io::Result<String> {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        }

        let scope = Scope::new(HashMap::new(), HashMap::new());
        let mut ctx = Context::new();

        let (scope, definitions) = {
            let std_content = load_library_file("core/index.ec").unwrap();
            let std_file = compile_str(&mut ctx, std_content.as_str()).unwrap();
            run(&mut ctx, scope, TypeDefinitions { definitions: Default::default() }, std_file, true).unwrap()
        };

        let mut path = PathBuf::from(file.clone());
        let content = load_text_from_file(path.to_str().unwrap()).unwrap();

        let source_file = compile_str(&mut ctx, content.as_str()).unwrap();

        let code = generate::generate_c_code(
            ir::Context {
                file: source_file,
                core_scope: scope,
                string_table: ctx.string_table,
                type_table: ctx.type_table
            }).unwrap();

        // println!("{}",code);

        build::build(
            file.file_name().unwrap().to_str().unwrap().replace(".ec", "").as_str(),
            &code,
        ).unwrap();

        return;
    }

    if args.get(1).unwrap() == "test" {
        test_files(
            vec![PathBuf::from(args.get(2).unwrap())],
            args.get(3).unwrap_or(&"true".to_string()) == "true",
            args.get(4).unwrap_or(&"false".to_string()) == "true",
        );
    } else {
        run_file(args.get(1).unwrap(), true);
    }
}


fn load_library_file(filename: &str) -> io::Result<String> {
    let manifest_dir = "/home/ddymke/repo/elodie/src/lib/";
    let file_path = PathBuf::from(manifest_dir).join(filename);

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


fn load_test_runner() -> io::Result<String> {
    let manifest_dir = "/home/ddymke/repo/elodie/src/test-runner/index.ec";
    let file_path = PathBuf::from(manifest_dir);

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
