#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![feature(inherent_associated_types)]

use std::fs::read_to_string;

use clap::{Parser, Subcommand};

pub mod assembler;
pub mod opcodes;
pub mod repl;
pub mod types;
pub mod vm;

#[derive(Parser, Debug)]
#[command(name = "rbvm")]
#[command(about = "RBVM (0.1.0) CLI", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run the assembler with a file
    Run {
        /// The file name to assemble
        #[arg(value_name = "FILE")]
        path: String,
    },
    /// Start the REPL
    Repl,
}

pub fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Run { path } => {
            let source_code = read_to_string(path).expect("File not found");
            let mut assembler = assembler::Assembler::default();
            let mut vm = vm::Vm::default();
            let bytecode = assembler.assemble(&source_code);
            if let Some(bytecode) = bytecode {
                vm.push_bytes(bytecode);
                vm.run();
            }
        }
        Commands::Repl => {
            let mut repl = repl::Repl::new();
            repl.run()
        }
    }
}
