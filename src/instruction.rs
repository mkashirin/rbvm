#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    EMTB,
    HLT,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ,
    GT,
    LT,
    GTQ,
    LTQ,
    JEQ,
    JNEQ,
    IGL,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::EMTB,
            1 => Opcode::HLT,
            2 => Opcode::LOAD,
            3 => Opcode::ADD,
            4 => Opcode::SUB,
            5 => Opcode::MUL,
            6 => Opcode::DIV,
            7 => Opcode::JMP,
            8 => Opcode::JMPF,
            9 => Opcode::JMPB,
            10 => Opcode::EQ,
            11 => Opcode::NEQ,
            12 => Opcode::GT,
            13 => Opcode::LT,
            14 => Opcode::GTQ,
            15 => Opcode::LTQ,
            16 => Opcode::JEQ,
            17 => Opcode::JNEQ,
            _ => Opcode::IGL,
        }
    }
}

#[allow(dead_code)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
