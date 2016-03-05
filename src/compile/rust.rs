use tempdir::TempDir;
use iron::prelude::*;
use iron::status;

use std::process::Command;

use std::env;
use std::io::{Read, Write};
use std::fs::File;

pub fn handle_rust(request: &mut Request) -> IronResult<Response> {

    println!("Request to compile Rust code");

    let mut code = String::new();
    request.body.read_to_string(&mut code)
        .expect("Could not read request body");

    let current_dir = env::current_dir()
        .expect("Can not determine current directory");

    let tempdir = TempDir::new(current_dir.to_str().unwrap())
        .expect("Could not create a temporary directory");

    let mut f = File::create(tempdir.path().join("rust.rs"))
        .expect("Could not create a file");

    f.write_all(code.as_bytes())
        .expect("Could not write to file");

    let rustc = Command::new("rustc")
                     .arg(tempdir.path().join("rust.rs"))
                     .arg("-o")
                     .arg(tempdir.path().join("rust"))
                     .output()
                     .unwrap_or_else(|e| { panic!("failed to compile: {}", e) });

    if !rustc.status.success() {
        let raw_err = String::from_utf8(rustc.stderr).unwrap();
        let mut err = String::new();

        let l = tempdir.path()
                       .join("rust.rs")
                       .to_str()
                       .unwrap()
                       .len();

        for line in raw_err.lines() {
            if line.len() > l {
                err.push_str(&line.split_at(l).1);
            } else {
                err.push_str(&line);
            }
            err.push_str("\n");
        }
        
        return Ok(Response::with((status::Ok, err)))
    }

    let program = Command::new(tempdir.path().join("rust"))
                     .output()
                     .unwrap_or_else(|e| { panic!("failed to execute program: {}", e) });

    Ok(Response::with((status::Ok, program.stdout)))
}
