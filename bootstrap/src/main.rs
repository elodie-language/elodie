use std::{env, io};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::run::run_file;

mod common;
mod cli;
mod ir;
mod compile;
mod run;
mod lex;
mod parse;
mod r#type;

fn main() {
    let args: Vec<String> = env::args().collect();
    run_file(args.get(1).unwrap());
}


fn load_library_file(filename: &str) -> io::Result<String> {
    let manifest_dir = "/home/ddymke/repo/elodie/src/lib/";
    let file_path = PathBuf::from(manifest_dir).join(filename);

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
