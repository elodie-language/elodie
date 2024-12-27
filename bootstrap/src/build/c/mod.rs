use std::{fs, io};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub use node::*;

use crate::build::c::emitter::emit;
use crate::build::c::generator::generate;
use crate::common::Context;
use crate::ir::ir_from_str;

mod emitter;
mod generator;
mod node;

#[derive(Debug)]
pub enum Error {
    // generator error
    // writer error
}

impl From<generator::Error> for Error {
    fn from(value: generator::Error) -> Self {
        todo!()
    }
}

impl From<emitter::Error> for Error {
    fn from(value: emitter::Error) -> Self {
        todo!()
    }
}

type Result<T> = core::result::Result<T, Error>;

pub fn build_c_code_from_file(file: PathBuf) {
    let name = file.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(".ec", "");

    let mut file = File::open(&file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut ctx = Context::new();
    let ir = ir_from_str(&mut ctx, content.as_str()).unwrap();
    let nodes = generate(ctx, ir).unwrap();
    let c_code = emit(&nodes);

    build(name.as_str(), c_code.as_str()).unwrap();
}


const EC_FILES: [&str; 6] = [
    "core/boolean.h",
    "core/boolean.c",
    "core/intrinsics/math.h",
    "core/intrinsics/math.c",
    "rt/intrinsics/io.h",
    "rt/intrinsics/io.c",
];

pub fn build(name: &str, c_code: &str) -> io::Result<()> {
    // FIXME needs context so that it know whichs core / std or lib to include

    let dir = PathBuf::from(format!("/tmp/elodie/{name}").as_str());

    let _ = fs::remove_dir_all(dir.clone());
    fs::create_dir_all(dir.clone()).unwrap();

    let c_file_path = dir.join(format!("main.c").as_str());
    let binary_path = dir.join(name);
    let gcc_err_path = dir.join("compiler.err");

    copy_sysroot(dir.clone());
    build_std(dir.clone());

    let mut c_file = File::create(&c_file_path)?;
    c_file.write_all(c_code.as_bytes())?;
    drop(c_file);

    let gcc_err_file = File::create(&gcc_err_path)?;

    let c_files: Vec<_> = EC_FILES
        .into_iter()
        .filter(|f| f.ends_with(".c"))
        .map(|f| dir.join(f))
        .collect();

    let gcc_output = Command::new("gcc")
        .arg(c_file_path.to_str().unwrap())
        .args(c_files)
        .arg(dir.join("rt/io.c"))
        .arg("-lm")
        .arg("-o")
        .arg(binary_path.to_str().unwrap())
        .stderr(Stdio::from(gcc_err_file))
        .output()?;

    if !gcc_output.status.success() {
        eprintln!("gcc failed with status: {}\n", gcc_output.status);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "gcc compilation failed",
        ));
    }

    Ok(())
}

fn build_std(dir: PathBuf) {
    let mut file = File::create(&dir.join(PathBuf::from("rt/io.h"))).unwrap();
    file.write_all(
        r#"
#ifndef RT_IO_H
#define RT_IO_H

void rt_io_print(char const * message);
void rt_io_println(char const * message);

#endif
    "#
            .as_bytes(),
    )
        .unwrap();
    drop(file);

    let mut file = File::create(&dir.join(PathBuf::from("rt/io.c"))).unwrap();
    file.write_all(
        r#"
#include "io.h"
#include "intrinsics/io.h"

void rt_io_print(char const * message) {
    rt_intrinsics_io_print(message);
}

void rt_io_println(char const * message) {
    rt_io_print(message);
    rt_io_print("\n");
}

    "#
            .as_bytes(),
    )
        .unwrap();
    drop(file);
}

fn copy_sysroot(destination: PathBuf) {
    let sys_root = "/home/ddymke/repo/elodie/src/sysroot/c";
    let file_path = PathBuf::from(sys_root);

    if !destination.exists() {
        fs::create_dir_all(&destination).unwrap();
    }

    for file in &EC_FILES {
        let source = file_path.join(file);
        let dest = destination.join(file);
        // Copy the file
        fs::create_dir_all(&dest.parent().unwrap()).unwrap();
        fs::copy(&source, &dest).unwrap();
        drop(dest)
    }
}
