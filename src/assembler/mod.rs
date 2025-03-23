use program_parser::program_parser;

use crate::opcodes::Opcode;

pub mod instruction_parsers;
pub mod opcode_parsers;
pub mod operand_parsers;
pub mod program_parser;

#[derive(Debug)]
pub enum Error {
    ParseError,
}

#[derive(Debug, Default)]
pub struct Assembler {
    program: Program,
    bytecode: Vec<u8>,
}
impl Assembler {
    pub fn assemble(&mut self, source_code: &str) -> Result<Vec<u8>, Error> {
        match program_parser(source_code) {
            Ok((_, program)) => Ok(self.emit_bytecode(program)),
            Err(_err) => Err(Error::ParseError),
        }
    }

    fn emit_bytecode(&mut self, program: Program) -> Vec<u8> {
        self.bytecode = program.to_bytes();
        self.program = program;
        self.bytecode.clone()
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Program {
    pub instrs: Vec<Instruction>,
}
impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instr in &self.instrs {
            let mut bytes = instr.to_bytes();
            program.append(&mut bytes);
        }
        program
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Instruction {
    opcode: MaybeToken,
    operands: (MaybeToken, MaybeToken, MaybeToken),
}
impl Instruction {
    pub fn new(
        opcode: MaybeToken,
        operands: (MaybeToken, MaybeToken, MaybeToken),
    ) -> Self {
        Self { opcode, operands }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut parsed = vec![];
        if let Some(Token::Op { code }) = &self.opcode {
            parsed.push(*code as u8);
        } else {
            println!("Invalid value found in opcode field");
            std::process::exit(1);
        };

        for operand in
            [&self.operands.0, &self.operands.1, &self.operands.2].iter()
        {
            if let Some(token) = *operand {
                Instruction::extract_operand(token, &mut parsed)
            }
        }
        while parsed.len() < 4 {
            parsed.push(0);
        }
        parsed
    }

    fn extract_operand(token: &Token, parsed: &mut Vec<u8>) {
        match token {
            Token::Register { index: reg_index } => {
                parsed.push(*reg_index);
            }
            Token::Integer { value } => {
                let converted = *value as u16;
                parsed.push((converted >> 8) as u8);
                parsed.push(converted as u8);
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        };
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Op { code: Opcode },
    Register { index: u8 },
    Integer { value: i32 },
}
type MaybeToken = Option<Token>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::Vm;

    #[test]
    fn test_assemble_program() {
        let mut assembler = Assembler::default();
        let program = r#"load $0 #100
load $1 #2
load $2 #1
inc $2
eq $1 $2
halt
        "#;
        let assembled = assembler.assemble(program).unwrap();
        let mut vm = Vm::default();
        assert_eq!(assembled.len(), 24);
        vm.push_bytes(assembled);
        assert_eq!(vm.program.len(), 24);
    }
}
