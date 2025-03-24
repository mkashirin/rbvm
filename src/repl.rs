use std::io;
use std::io::Write;

use crate::assembler::program_parser::*;
use crate::vm::{Error, Vm};

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Repl {
    vm: Vm,
    command_buffer: Vec<String>,
}

impl Repl {
    pub fn run(&mut self) -> Result<Vm, Error> {
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
                "!exit" => return Ok(self.vm.clone()),
                "!buffer" => self.print_command_buffer(),
                "!registers" => println!("Registers: {:?}", self.vm.registers),
                _ => self.process_line(buffer),
            }
        }
    }

    fn print_command_buffer(&self) {
        println!("Command buffer:");
        for command in &self.command_buffer {
            println!("{}", command)
        }
    }

    fn process_line(&mut self, buffer: &str) {
        let parsed_program = program_parser(buffer);
        if let Err(_err) = &parsed_program {
            eprintln!("Instruction not parsed. Resuming...");
            return;
        }
        let (_, result) = parsed_program.unwrap();
        let bytecode = result.to_bytes();
        if let Ok(bytecode) = bytecode {
            for byte in bytecode {
                self.vm.push_byte(byte);
            }
            if let Err(Error::IllegalOpcode) = self.vm.run_once() {
                eprintln!("Illegal opcode found. Resuming...");
            }
        }
    }
}
