use nom::branch::alt;
use nom::combinator::map;
use nom::{IResult, Parser};

use super::opcode_parser::*;
use super::operand_parsers::{oop, operand_parser};
#[allow(unused_imports)]
use super::{Instruction, MaybeToken, Token};

pub fn instr_parser0(input: &str) -> IResult<&str, Instruction> {
    let combined = (opcode_parser, operand_parser, oop);
    map(combined, |(opcode, op0, op1)| Instruction {
        opcode: Some(opcode),
        operands: (Some(op0), op1, None),
    })
    .parse(input)
}

pub fn instr_parser1(input: &str) -> IResult<&str, Instruction> {
    let combined = (opcode_parser, oop, oop, oop);
    map(combined, |(opcode, op0, op1, op2)| Instruction {
        opcode: Some(opcode),
        operands: (op0, op1, op2),
    })
    .parse(input)
}

pub fn instr_parser(input: &str) -> IResult<&str, Instruction> {
    alt((instr_parser1, instr_parser0)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opcodes::Opcode;

    #[test]
    fn test_instr_parser0_one_register() {
        let result = instr_parser0("jump $0");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            Instruction::new(
                Some(Token::Op { code: Opcode::JUMP }),
                (Some(Token::Register { index: 0 }), None, None),
            )
        );
    }

    #[test]
    fn test_instr_parser0_register_with_int() {
        let result = instr_parser0("load $0 #12");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            Instruction::new(
                Some(Token::Op { code: Opcode::LOAD }),
                (
                    Some(Token::Register { index: 0 }),
                    Some(Token::Integer { value: 12 }),
                    None
                ),
            )
        );
    }

    #[test]
    fn test_instr_parser1_no_registers() {
        let result = instr_parser1("halt");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            Instruction::new(
                Some(Token::Op { code: Opcode::HALT }),
                (None, None, None),
            )
        );
    }

    #[test]
    fn test_instr_parser1_two_registers() {
        let result = instr_parser1("ne $1 $24");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            Instruction::new(
                Some(Token::Op { code: Opcode::NE }),
                (
                    Some(Token::Register { index: 1 }),
                    Some(Token::Register { index: 24 }),
                    None
                )
            )
        );
    }

    #[test]
    fn test_instr_parser1_three_registers() {
        let result = instr_parser1("mul $1 $28 $3");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            Instruction::new(
                Some(Token::Op { code: Opcode::MUL }),
                (
                    Some(Token::Register { index: 1 }),
                    Some(Token::Register { index: 28 }),
                    Some(Token::Register { index: 3 }),
                )
            )
        );
    }
}
