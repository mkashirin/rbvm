use std::fs::{File, read_to_string};
use std::io;
use std::io::Write;
use std::num::ParseIntError;
use std::path::Path;

use crate::assembler::program_parser::parse_program;
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
                "!history" => {
                    for command in &self.command_buffer {
                        println!("{}", command)
                    }
                }
                "!program" => {
                    println!("VM program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                }
                "!registers" => {
                    println!("VM registers:");
                    println!("{:#?}", self.vm.registers);
                }
                "!load" => {
                    print!("Enter path to the file to load: ");
                    io::stdout().flush().expect("Unable to flush stdout");
                    let mut buffer = String::new();
                    stdin
                        .read_line(&mut buffer)
                        .expect("Unable to read line from user");
                    let buffer = buffer.trim();
                    let filename = Path::new(&buffer);
                    File::open(Path::new(&filename)).expect("File not found");
                    let contents = read_to_string(filename)
                        .expect("There was an error reading from the file");
                    let program = match parse_program(&contents) {
                        Ok((_, program)) => program,
                        Err(e) => {
                            println!("Unable to parse input: {:#?}", e);
                            continue;
                        }
                    };
                    self.vm.program.append(&mut program.to_bytes());
                }
                _ => {
                    // Code for parsing hexadecimal if you need it:
                    // ```
                    // let parsed = self.parse_hex(buffer);
                    // match parsed {
                    //     Ok(bytes) => {
                    //         for value in bytes {
                    //             self.vm.push_byte(value);
                    //         }
                    //     }
                    //     Err(_) => println!("Unable to decode hex"),
                    // };
                    // self.vm.run_once();
                    // ```
                    let parsed_program = parse_program(buffer);
                    if parsed_program.is_err() {
                        println!("Unable to parse input");
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

    fn _parse_hex(&self, buffer: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = buffer.split(" ").collect::<Vec<&str>>();
        let mut parsed: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(hex_string, 16);
            match byte {
                Ok(value) => {
                    parsed.push(value);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(parsed)
    }
}
