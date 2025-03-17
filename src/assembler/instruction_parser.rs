use nom::{
    branch::alt,
    combinator::{map, opt},
    IResult, Parser,
};

use super::{
    opcode_parser::*, operand_parser::parse_integer_operand,
    register_parser::parse_register, Token,
};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand0: Option<Token>,
    operand1: Option<Token>,
    operand2: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut parsed = vec![];
        match &self.opcode {
            Token::Op { code } => parsed.push(*code as u8),
            _ => {
                println!("Invalid value found in opcode field");
                std::process::exit(1);
            }
        };

        for operand in [&self.operand0, &self.operand1, &self.operand2].iter() {
            if let Some(token) = *operand {
                AssemblerInstruction::extract_operand(token, &mut parsed)
            }
        }
        parsed
    }

    fn extract_operand(token: &Token, parsed: &mut Vec<u8>) {
        match token {
            Token::Register { reg_index } => {
                parsed.push(*reg_index);
            }
            Token::IntegerOperand { value } => {
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

pub fn parse_instruction0(input: &str) -> IResult<&str, AssemblerInstruction> {
    map(
        (
            parse_opcode,
            opt(parse_register),
            opt(parse_integer_operand),
        ),
        |(opcode, operand0, operand1)| AssemblerInstruction {
            opcode,
            operand0,
            operand1,
            operand2: None,
        },
    )
    .parse(input)
}

pub fn parse_instruction1(input: &str) -> IResult<&str, AssemblerInstruction> {
    map(
        (
            parse_opcode,
            parse_register,
            parse_register,
            opt(parse_register),
        ),
        |(opcode, operand0, operand1, operand2)| AssemblerInstruction {
            opcode,
            operand0: Some(operand0),
            operand1: Some(operand1),
            operand2,
        },
    )
    .parse(input)
}

pub fn parse_instruction(input: &str) -> IResult<&str, AssemblerInstruction> {
    alt((parse_instruction1, parse_instruction0)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction0_no_registers() {
        let result = parse_instruction0("hlt");
        assert!(result.is_ok());
        let (_, instruction) = result.unwrap();
        assert_eq!(
            instruction,
            AssemblerInstruction {
                opcode: Token::Op { code: Opcode::HLT },
                operand0: None,
                operand1: None,
                operand2: None,
            }
        );
    }

    #[test]
    fn test_parse_instruction0_one_register() {
        let result = parse_instruction0("jmp $0");
        assert!(result.is_ok());
        let (_, instruction) = result.unwrap();
        assert_eq!(
            instruction,
            AssemblerInstruction {
                opcode: Token::Op { code: Opcode::JMP },
                operand0: Some(Token::Register { reg_index: 0 }),
                operand1: None,
                operand2: None,
            }
        );
    }

    #[test]
    fn test_parse_instruction0_register_with_integer() {
        let result = parse_instruction0("load $0 #102");
        assert!(result.is_ok());
        let (_, instruction) = result.unwrap();
        assert_eq!(
            instruction,
            AssemblerInstruction {
                opcode: Token::Op { code: Opcode::LOAD },
                operand0: Some(Token::Register { reg_index: 0 }),
                operand1: Some(Token::IntegerOperand { value: 102 }),
                operand2: None,
            }
        );
    }

    #[test]
    fn test_parse_instruction1_two_registers() {
        let result = parse_instruction1("neq $1 $2");
        assert!(result.is_ok());
        let (_, instruction) = result.unwrap();
        assert_eq!(
            instruction,
            AssemblerInstruction {
                opcode: Token::Op { code: Opcode::NEQ },
                operand0: Some(Token::Register { reg_index: 1 }),
                operand1: Some(Token::Register { reg_index: 2 }),
                operand2: None,
            }
        );
    }

    #[test]
    fn test_parse_instruction1_three_registers() {
        let result = parse_instruction1("mul $1 $2 $3");
        assert!(result.is_ok());
        let (_, instruction) = result.unwrap();
        assert_eq!(
            instruction,
            AssemblerInstruction {
                opcode: Token::Op { code: Opcode::MUL },
                operand0: Some(Token::Register { reg_index: 1 }),
                operand1: Some(Token::Register { reg_index: 2 }),
                operand2: Some(Token::Register { reg_index: 3 }),
            }
        );
    }
}
