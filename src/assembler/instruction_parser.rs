use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::{IResult, Parser};

use super::opcode_parser::*;
use super::operand_parser::parse_operand;
use super::{MaybeToken, Token};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstr {
    opcode: MaybeToken,
    label: MaybeToken,
    directive: MaybeToken,
    operand0: MaybeToken,
    operand1: MaybeToken,
    operand2: MaybeToken,
}

impl AssemblerInstr {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut parsed = vec![];
        if let Some(opcode) = &self.opcode {
            match opcode {
                Token::Op { code } => parsed.push(*code as u8),
                _ => {
                    println!("Invalid value found in opcode field");
                    std::process::exit(1);
                }
            }
        };

        for operand in [&self.operand0, &self.operand1, &self.operand2].iter() {
            if let Some(token) = *operand {
                AssemblerInstr::extract_operand(token, &mut parsed)
            }
        }
        while parsed.len() < 4 {
            parsed.push(0);
        }
        parsed
    }

    fn extract_operand(token: &Token, parsed: &mut Vec<u8>) {
        match token {
            Token::Register { reg_index } => {
                parsed.push(*reg_index);
            }
            Token::IntOperand { value } => {
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

pub fn parse_instr0(input: &str) -> IResult<&str, AssemblerInstr> {
    map(
        (parse_opcode, opt(parse_operand), opt(parse_operand)),
        |(opcode, operand0, operand1)| AssemblerInstr {
            opcode: Some(opcode),
            label: None,
            directive: None,
            operand0,
            operand1,
            operand2: None,
        },
    )
    .parse(input)
}

pub fn parse_instr1(input: &str) -> IResult<&str, AssemblerInstr> {
    map(
        (
            parse_opcode,
            parse_operand,
            parse_operand,
            opt(parse_operand),
        ),
        |(opcode, operand0, operand1, operand2)| AssemblerInstr {
            opcode: Some(opcode),
            label: None,
            directive: None,
            operand0: Some(operand0),
            operand1: Some(operand1),
            operand2,
        },
    )
    .parse(input)
}

pub fn parse_instr(input: &str) -> IResult<&str, AssemblerInstr> {
    alt((parse_instr1, parse_instr0)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction0_no_registers() {
        let result = parse_instr0("hlt");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            AssemblerInstr {
                opcode: Some(Token::Op { code: Opcode::HLT }),
                label: None,
                directive: None,
                operand0: None,
                operand1: None,
                operand2: None,
            }
        );
    }

    #[test]
    fn test_parse_instruction0_one_register() {
        let result = parse_instr0("jmp $0");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            AssemblerInstr {
                opcode: Some(Token::Op { code: Opcode::JMP }),
                label: None,
                directive: None,
                operand0: Some(Token::Register { reg_index: 0 }),
                operand1: None,
                operand2: None,
            }
        );
    }

    #[test]
    fn test_parse_instruction0_register_with_int() {
        let result = parse_instr0("load $0 #102");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            AssemblerInstr {
                opcode: Some(Token::Op { code: Opcode::LOAD }),
                label: None,
                directive: None,
                operand0: Some(Token::Register { reg_index: 0 }),
                operand1: Some(Token::IntOperand { value: 102 }),
                operand2: None,
            }
        );
    }

    #[test]
    fn test_parse_instruction1_two_registers() {
        let result = parse_instr1("neq $1 $2");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            AssemblerInstr {
                opcode: Some(Token::Op { code: Opcode::NEQ }),
                label: None,
                directive: None,
                operand0: Some(Token::Register { reg_index: 1 }),
                operand1: Some(Token::Register { reg_index: 2 }),
                operand2: None,
            }
        );
    }

    #[test]
    fn test_parse_instruction1_three_registers() {
        let result = parse_instr1("mul $1 $2 $3");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            AssemblerInstr {
                opcode: Some(Token::Op { code: Opcode::MUL }),
                label: None,
                directive: None,
                operand0: Some(Token::Register { reg_index: 1 }),
                operand1: Some(Token::Register { reg_index: 2 }),
                operand2: Some(Token::Register { reg_index: 3 }),
            }
        );
    }
}
