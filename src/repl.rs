use std::io;
use std::io::Write;

use crate::assembler::program_parser::*;
use crate::vm::Vm;

#[allow(dead_code)]
#[derive(Default)]
pub struct Repl {
    vm: Vm,
    command_buffer: Vec<String>,
}

impl Repl {
    pub fn new() -> Repl {
        Repl::default()
    }

    pub fn run(&mut self) {
        println!("RBVM (0.1.0) REPL");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();
            print!(":>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            stdin
                .read_line(&mut buffer)
                .expect("Unable to read input from user");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                "!exit" => {
                    println!("Exiting REPL...");
                    std::process::exit(0);
                }
                "!hist" => {
                    println!("Command buffer:");
                    for command in &self.command_buffer {
                        println!("{}", command)
                    }
                }
                "!prog" => {
                    println!("VM program vector:");
                    for instr in &self.vm.program {
                        println!("{}", instr);
                    }
                }
                "!reg" => {
                    println!("VM registers:\n{:#?}", self.vm.registers);
                }
                _ => {
                    let parsed_program = program_parser(buffer);
                    if let Err(err) = parsed_program {
                        println!("Unable to parse input: {:?}", err);
                        continue;
                    }
                    let (_, result) = parsed_program.unwrap();
                    let bytecode = result.to_bytes();
                    for byte in bytecode {
                        self.vm.push_byte(byte);
                    }
                    self.vm.run_once();
                }
            }
        }
    }
}
