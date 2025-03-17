use crate::instruction::Opcode;

pub mod instruction_parser;
pub mod opcode_parser;
pub mod operand_parser;
pub mod program_parser;
pub mod special_symbol_parser;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_index: u8 },
    IntOperand { value: i32 },
    LabelDecl { name: String },
    LabelUsage { name: String },
    Directive { name: String },
}
