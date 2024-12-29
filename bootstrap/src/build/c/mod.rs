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
mod scope;

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


const EC_FILES: [&str; 2] = [
    "rt/include/io.h",
    "rt/src/io.c",
];

pub fn build(name: &str, c_code: &str) -> io::Result<()> {
    // FIXME needs context so that it know whichs core / std or lib to include

    let dir = PathBuf::from(format!("/tmp/elodie/{name}").as_str());

    let _ = fs::remove_dir_all(dir.clone());
    fs::create_dir_all(dir.clone()).unwrap();

    let c_file_path = dir.join(format!("main.c").as_str());
    let binary_path = dir.join(name);
    let gcc_err_path = dir.join("compiler.err");

    // copy_sysroot(dir.clone());
    build_std(dir.clone());

    let mut c_file = File::create(&c_file_path)?;
    c_file.write_all(c_code.as_bytes())?;
    drop(c_file);

    let gcc_err_file = File::create(&gcc_err_path)?;

    // let c_files: Vec<_> = EC_FILES
    //     .into_iter()
    //     .filter(|f| f.ends_with(".c"))
    //     .map(|f| dir.join("sysroot").join(f))
    //     .collect();

    let gcc_output = Command::new("gcc")
        .arg(c_file_path.to_str().unwrap())
        .arg(dir.join("rt/io.c"))
        .arg("-std=gnu2x")
        .arg("-I/home/ddymke/repo/elodie/src/sysroot/c/project/core/include")
        .arg("-L/home/ddymke/repo/elodie/src/sysroot/c/build/project/core")
        .arg("-lcore")
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
    fs::create_dir_all(dir.join("rt")).unwrap();
    let mut file = File::create(&dir.join(PathBuf::from("rt/io.h"))).unwrap();
    file.write_all(
        r#"
#ifndef RT_IO_H
#define RT_IO_H

void rt_io_print(char const * message);
void rt_io_println(char const * message);

void test();

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

void rt_io_print(char const * message) {
//    sysroot_rt_io_print(message);
}

void rt_io_println(char const * message) {
  //  rt_io_print(message);
  //  rt_io_print("\n");
}

#include "core/core-api.h"


void test(){
	u1 input_array[] = {'H', 'A', 'M', 'A', 'L'};

	auto tm = mem_test_new_default (128);
		struct bytes_view bytes = {
		.data = input_array,
		.size = 2
	};

	struct string *result = string_allocate_from_bytes (MEM(tm), bytes);

//	string_deallocate (result, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

    "#
            .as_bytes(),
    )
        .unwrap();
    drop(file);
}
