use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::{IResult, Parser};

use super::instruction_parsers::{AssemblerInstr, instr_parser};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub instrs: Vec<AssemblerInstr>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instrs {
            program.append(&mut instruction.to_bytes());
        }
        program
    }
}

pub fn program_parser(input: &str) -> IResult<&str, Program> {
    map(
        many1(alt((terminated(instr_parser, tag("\n")), instr_parser))),
        |instrs| Program { instrs },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_parser_no_registers() {
        let result = program_parser("hlt\n");
        assert!(result.is_ok());
        let (leftover, program) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(1, program.instrs.len());
    }

    #[test]
    fn test_program_parser_one_register() {
        let result = program_parser("jmp $0\n");
        assert!(result.is_ok());
        let (leftover, program) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(1, program.instrs.len());
    }

    #[test]
    fn test_program_parser_two_registers() {
        let result = program_parser("eq $1 $2\n");
        assert!(result.is_ok());
        let (leftover, program) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(1, program.instrs.len());
    }

    #[test]
    fn test_program_parser_register_with_integer() {
        let result = program_parser("load $0 #102\n");
        assert!(result.is_ok());
        let (leftover, program) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(1, program.instrs.len());
    }

    #[test]
    fn test_program_parser_three_registers() {
        let result = program_parser("add $1 $2 $3\n");
        assert!(result.is_ok());
        let (leftover, program) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(1, program.instrs.len());
    }

    #[test]
    fn test_program_parser_to_bytes() {
        let result = program_parser("load $0 #102\n");
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
    }
}
