// Copyright (c) 2019 King's College London created by the Software Development
// Team <http://soft-dev.org/>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, or the UPL-1.0 license
// <http://opensource.org/licenses/UPL> at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::{
    path::PathBuf,
    process::{self, Command},
};


fn compile() -> PathBuf {
    println!("Compiling...");
    let mut input = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    input.push("src");
    input.push("child.rs");

    let mut output = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    output.push("childbin");

    let mut compiler = Command::new("rustc");

    compiler.args(&[
                  "--edition=2018",
                  "-o",
                  output.to_str().unwrap(),
                  input.to_str().unwrap(),
    ]);

    compiler
        .stderr(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .output()
        .unwrap();
    output
}

fn main() {
     let output = compile();

     println!("Running bin...");

     let mut runtime = Command::new(output);
    
     let mut child = runtime
         .stderr(process::Stdio::piped())
         .stdout(process::Stdio::piped())
         .spawn()
         .unwrap();

     // This deadlocks
     child.wait().unwrap();

}
