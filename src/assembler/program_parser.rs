use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::{IResult, Parser};

use super::Program;
use super::instruction_parsers::instr_parser;

pub fn program_parser(input: &str) -> IResult<&str, Program> {
    let with_newline = terminated(instr_parser, tag("\n"));
    let combined = alt((with_newline, instr_parser));
    map(many1(combined), |instrs| Program { instrs }).parse(input)
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
        assert_eq!(program.instrs.len(), 1);
    }

    #[test]
    fn test_program_parser_one_register() {
        let result = program_parser("jmp $0\n");
        assert!(result.is_ok());
        let (leftover, program) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(program.instrs.len(), 1);
    }

    #[test]
    fn test_program_parser_two_registers() {
        let result = program_parser("eq $1 $2\n");
        assert!(result.is_ok());
        let (leftover, program) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(program.instrs.len(), 1);
    }

    #[test]
    fn test_program_parser_register_with_integer() {
        let result = program_parser("load $0 #102\n");
        assert!(result.is_ok());
        let (leftover, program) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(program.instrs.len(), 1);
    }

    #[test]
    fn test_program_parser_three_registers() {
        let result = program_parser("add $1 $2 $3\n");
        assert!(result.is_ok());
        let (leftover, program) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(program.instrs.len(), 1);
    }

    #[test]
    fn test_program_parser_to_bytes() {
        let result = program_parser("load $0 #102\n");
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
    }
}
