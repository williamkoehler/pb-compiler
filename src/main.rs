#![allow(dead_code)]

use std::{
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use clap::Parser;

mod compiler;

#[derive(Parser)]
enum Cli {
    Build {
        path: String,

        #[clap(short, long)]
        cpp: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::Build { path, cpp } => {
            let path = PathBuf::from(path);

            // Compile
            let input = {
                let mut file = std::fs::File::open(path)
                    .with_context(|| format!("Failed to open input file"))?;
                let mut input = String::new();
                file.read_to_string(&mut input)
                    .with_context(|| format!("Failed to read from input file"))?;
                input
            };

            let mut compiler = compiler::Compiler::new();
            let file = compiler.compile(&input).with_context(|| {
                format!(
                    "{}",
                    compiler
                        .reports()
                        .iter()
                        .fold(String::new(), |acc, r| acc + &r.with_source_code(&input))
                )
            })?;

            // Generate cpp
            if let Some(output_path) = cpp {
                let output_path = PathBuf::from(output_path);

                let mut output_file = std::fs::File::create(output_path)
                    .with_context(|| format!("Failed to open output file"))?;

                let generator = compiler::generator::Generator::from(file);
                let code = generator.generate_cplusplus();

                output_file
                    .write(code.as_bytes())
                    .with_context(|| format!("Failed to write to output file"))?;
            }

            Ok(())
        }
    }
}
