use std::{env, fs};
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn build(name: &str, c_code: &str) -> io::Result<()> {
    let dir = PathBuf::from(format!("/tmp/elodie/{name}").as_str());

    let _ = fs::remove_dir_all(dir.clone());
    fs::create_dir_all(dir.clone()).unwrap();

    let c_file_path = dir.join(format!("{name}.c").as_str());
    let binary_path = dir.join(name);
    let gcc_err_path = dir.join("compiler.err");

    let mut c_file = File::create(&c_file_path)?;
    c_file.write_all(c_code.as_bytes())?;
    drop(c_file);

    let gcc_err_file = File::create(&gcc_err_path)?;

    let gcc_output = Command::new("gcc")
        .arg(c_file_path.to_str().unwrap())
        .arg("-o")
        .arg(binary_path.to_str().unwrap())
        .stderr(Stdio::from(gcc_err_file))
        .output()?;

    if !gcc_output.status.success() {
        eprintln!(
            "gcc failed with status: {}\nstderr:\n{}",
            gcc_output.status,
            String::from_utf8_lossy(&gcc_output.stderr)
        );
        return Err(io::Error::new(io::ErrorKind::Other, "gcc compilation failed"));
    }

    Ok(())
}