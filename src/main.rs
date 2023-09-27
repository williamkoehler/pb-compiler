#![allow(dead_code)]

use std::io::Read;

mod compiler;

fn main() -> Result<(), ()> {
    let mut file = std::fs::File::open("test.msg").expect("no such file");
    let mut input = String::new();
    _ = file.read_to_string(&mut input);

    let mut compiler = compiler::Compiler::new();
    match compiler.compile(&input) {
        Ok(_) => {
            // println!("{}", message);
        }
        Err(_) => {
            for report in compiler.reports() {
                println!("{}", report.with_source_code(&input));
            }
        }
    }

    Ok(())
}
