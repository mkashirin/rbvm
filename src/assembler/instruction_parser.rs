use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::{IResult, Parser};

use super::opcode_parser::*;
use super::operand_parser::parse_operand;
use super::symbol_parser::{parse_label_decl, parse_label_usage};
use super::{AssemblerError, MaybeToken, Token};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstr {
    directive: MaybeToken,
    label: MaybeToken,
    opcode: MaybeToken,
    operands: (MaybeToken, MaybeToken, MaybeToken),
}
impl AssemblerInstr {
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
        if let Some(opcode) = &self.opcode {
            match opcode {
                Token::Op { code } => parsed.push(*code as u8),
                _ => {
                    println!("Invalid value found in opcode field");
                    std::process::exit(1);
                }
            }
        };

        for operand in
            [&self.operands.0, &self.operands.1, &self.operands.2].iter()
        {
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
        (parse_opcode, parse_operand, opt(parse_operand)),
        |(opcode, operand0, operand1)| AssemblerInstr {
            directive: None,
            label: None,
            opcode: Some(opcode),
            operands: (Some(operand0), operand1, None),
        },
    )
    .parse(input)
}

pub fn parse_instr1(input: &str) -> IResult<&str, AssemblerInstr> {
    map(
        (
            opt(parse_label_decl),
            parse_opcode,
            opt(parse_operand),
            opt(parse_operand),
            opt(parse_operand),
        ),
        |(label, opcode, operand0, operand1, operand2)| AssemblerInstr {
            directive: None,
            label,
            opcode: Some(opcode),
            operands: (operand0, operand1, operand2),
        },
    )
    .parse(input)
}

pub fn parse_instr2(input: &str) -> IResult<&str, AssemblerInstr> {
    map((parse_opcode, parse_label_usage), |(opcode, label)| {
        AssemblerInstr {
            directive: None,
            label: Some(label),
            opcode: Some(opcode),
            operands: (None, None, None),
        }
    })
    .parse(input)
}

pub fn parse_instr(input: &str) -> IResult<&str, AssemblerInstr> {
    alt((parse_instr2, parse_instr1, parse_instr0)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instr1_no_registers() {
        let result = parse_instr1("hlt");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            AssemblerInstr::new(
                None,
                None,
                Some(Token::Op { code: Opcode::HLT }),
                (None, None, None),
            )
        );
    }

    #[test]
    fn test_parse_instr0_one_register() {
        let result = parse_instr0("jmp $0");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            AssemblerInstr::new(
                None,
                None,
                Some(Token::Op { code: Opcode::JMP }),
                (Some(Token::Register { reg_index: 0 }), None, None),
            )
        );
    }

    #[test]
    fn test_parse_instr0_register_with_int() {
        let result = parse_instr0("load $0 #12");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            AssemblerInstr::new(
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
    fn test_parse_instr1_two_registers() {
        let result = parse_instr1("neq $1 $24");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            AssemblerInstr::new(
                None,
                None,
                Some(Token::Op { code: Opcode::NEQ }),
                (
                    Some(Token::Register { reg_index: 1 }),
                    Some(Token::Register { reg_index: 24 }),
                    None
                )
            )
        );
    }

    #[test]
    fn test_parse_instr1_three_registers() {
        let result = parse_instr1("mul $1 $28 $3");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            AssemblerInstr::new(
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
    fn test_parse_instr1_label_decl_no_registers() {
        let result = parse_instr1("test: hlt");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            AssemblerInstr::new(
                None,
                Some(Token::LabelDecl {
                    name: "test".to_string()
                }),
                Some(Token::Op { code: Opcode::HLT }),
                (None, None, None),
            )
        );
    }

    #[test]
    fn test_parse_instr2_label_usage_no_registers() {
        let result = parse_instr2("jmp @test");
        assert!(result.is_ok());
        let (_, instr) = result.unwrap();
        assert_eq!(
            instr,
            AssemblerInstr::new(
                None,
                Some(Token::LabelUsage {
                    name: "test".to_string()
                }),
                Some(Token::Op { code: Opcode::JMP }),
                (None, None, None,),
            )
        );
    }
}
