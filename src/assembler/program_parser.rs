use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::many1,
    sequence::terminated, IResult, Parser,
};

use super::instruction_parser::{parse_instruction, AssemblerInstruction};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes());
        }
        program
    }
}

pub fn parse_program(input: &str) -> IResult<&str, Program> {
    map(
        many1(alt((
            terminated(parse_instruction, tag("\n")),
            parse_instruction,
        ))),
        |instructions| Program { instructions },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program_no_registers() {
        let result = parse_program("hlt\n");
        assert!(result.is_ok());
        let (leftover, program) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(1, program.instructions.len());
    }

    #[test]
    fn test_parse_program_one_register() {
        let result = parse_program("jmp $0\n");
        assert!(result.is_ok());
        let (leftover, program) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(1, program.instructions.len());
    }

    #[test]
    fn test_parse_program_two_registers() {
        let result = parse_program("eq $1 $2\n");
        assert!(result.is_ok());
        let (leftover, program) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(1, program.instructions.len());
    }

    #[test]
    fn test_parse_program_register_with_integer() {
        let result = parse_program("load $0 #102\n");
        assert!(result.is_ok());
        let (leftover, program) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(1, program.instructions.len());
    }

    #[test]
    fn test_parse_program_three_registers() {
        let result = parse_program("add $1 $2 $3\n");
        assert!(result.is_ok());
        let (leftover, program) = result.unwrap();
        assert_eq!(leftover, "");
        assert_eq!(1, program.instructions.len());
    }

    #[test]
    fn test_parse_program_load_to_bytes() {
        let result = parse_program("load $0 #102\n");
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
    }
}
