use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::{IResult, Parser};

use super::opcode_parsers::*;
use super::operand_parsers::operand_porser;
use super::symbol_parsers::{label_decl_parser, label_usage_parser};
use super::{AssemblerError, MaybeToken, Token};

#[derive(Debug, PartialEq)]
pub struct Instruction {
    directive: MaybeToken,
    label: MaybeToken,
    opcode: MaybeToken,
    operands: (MaybeToken, MaybeToken, MaybeToken),
}
impl Instruction {
    pub fn new(
        directive: MaybeToken,
        label: MaybeToken,
        opcode: MaybeToken,
        operands: (MaybeToken, MaybeToken, MaybeToken),
    ) -> Self {
        Self {
            directive,
            label,
            opcode,
            operands,
        }
    }

    pub fn try_exrtact_label(&self) -> Result<String, AssemblerError> {
        if let Some(Token::LabelDecl { name }) = self.label.clone() {
            Ok(name)
        } else {
            Err(AssemblerError::LabelNotExtracted)
        }
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

pub fn instr_parser0(input: &str) -> IResult<&str, Instruction> {
    map(
        (opcode_parser, operand_porser, opt(operand_porser)),
        |(opcode, operand0, operand1)| Instruction {
            directive: None,
            label: None,
            opcode: Some(opcode),
            operands: (Some(operand0), operand1, None),
        },
    )
    .parse(input)
}

pub fn instr_parser1(input: &str) -> IResult<&str, Instruction> {
    map(
        (
            opt(label_decl_parser),
            opcode_parser,
            opt(operand_porser),
            opt(operand_porser),
            opt(operand_porser),
        ),
        |(label, opcode, operand0, operand1, operand2)| Instruction {
            directive: None,
            label,
            opcode: Some(opcode),
            operands: (operand0, operand1, operand2),
        },
    )
    .parse(input)
}

pub fn instr_parser2(input: &str) -> IResult<&str, Instruction> {
    map((opcode_parser, label_usage_parser), |(opcode, label)| {
        Instruction {
            directive: None,
            label: Some(label),
            opcode: Some(opcode),
            operands: (None, None, None),
        }
    })
    .parse(input)
}

pub fn instr_parser(input: &str) -> IResult<&str, Instruction> {
    alt((instr_parser2, instr_parser1, instr_parser0)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_instr_parser0_one_register() {
        let result = instr_parser0("jump $0");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            Instruction::new(
                None,
                None,
                Some(Token::Op { code: Opcode::JUMP }),
                (Some(Token::Register { reg_index: 0 }), None, None),
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
                None,
                None,
                Some(Token::Op { code: Opcode::LOAD }),
                (
                    Some(Token::Register { reg_index: 0 }),
                    Some(Token::IntOperand { value: 12 }),
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
                None,
                None,
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
                None,
                None,
                Some(Token::Op { code: Opcode::NE }),
                (
                    Some(Token::Register { reg_index: 1 }),
                    Some(Token::Register { reg_index: 24 }),
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
                None,
                None,
                Some(Token::Op { code: Opcode::MUL }),
                (
                    Some(Token::Register { reg_index: 1 }),
                    Some(Token::Register { reg_index: 28 }),
                    Some(Token::Register { reg_index: 3 }),
                )
            )
        );
    }

    #[test]
    fn test_instr_parser1_label_decl_no_registers() {
        let result = instr_parser1("test: halt");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            Instruction::new(
                None,
                Some(Token::LabelDecl {
                    name: "test".to_string()
                }),
                Some(Token::Op { code: Opcode::HALT }),
                (None, None, None),
            )
        );
    }

    #[test]
    fn test_instr_parser2_label_usage_no_registers() {
        let result = instr_parser2("jump @test");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            Instruction::new(
                None,
                Some(Token::LabelUsage {
                    name: "test".to_string()
                }),
                Some(Token::Op { code: Opcode::JUMP }),
                (None, None, None,),
            )
        );
    }
}
