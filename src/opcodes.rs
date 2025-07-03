#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    PAD,
    HALT,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JUMP,
    JF,
    JB,
    EQ,
    NE,
    GT,
    LT,
    GTE,
    LTE,
    JE,
    JNE,
    INC,
    DEC,
    ILL,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        #[rustfmt::skip]
        match value {
            0   => Opcode::PAD,
            1   => Opcode::HALT,
            2   => Opcode::LOAD,
            3   => Opcode::ADD,
            4   => Opcode::SUB,
            5   => Opcode::MUL,
            6   => Opcode::DIV,
            7   => Opcode::JUMP,
            8   => Opcode::JF,
            9   => Opcode::JB,
            10  => Opcode::EQ,
            11  => Opcode::NE,
            12  => Opcode::GT,
            13  => Opcode::LT,
            14  => Opcode::GTE,
            15  => Opcode::LTE,
            16  => Opcode::JE,
            17  => Opcode::JNE,
            18  => Opcode::INC,
            19  => Opcode::DEC,
            _   => Opcode::ILL,
        }
    }
}

impl From<&str> for Opcode {
    fn from(value: &str) -> Self {
        #[rustfmt::skip]
        match value {
            "pad"   => Opcode::PAD,
            "halt"  => Opcode::HALT,
            "load"  => Opcode::LOAD,
            "add"   => Opcode::ADD,
            "sub"   => Opcode::SUB,
            "mul"   => Opcode::MUL,
            "div"   => Opcode::DIV,
            "jump"  => Opcode::JUMP,
            "jf"    => Opcode::JF,
            "jb"    => Opcode::JB,
            "eq"    => Opcode::EQ,
            "ne"    => Opcode::NE,
            "gt"    => Opcode::GT,
            "lt"    => Opcode::LT,
            "gte"   => Opcode::GTE,
            "lte"   => Opcode::LTE,
            "je"    => Opcode::JE,
            "jne"   => Opcode::JNE,
            "inc"   => Opcode::INC,
            "dec"   => Opcode::DEC,
            _       => Opcode::ILL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_halt() {
        let opcode = Opcode::HALT;
        assert_eq!(opcode, Opcode::HALT);
    }

    #[test]
    fn test_opcode_from_str() {
        let opcode = Opcode::from("load");
        assert_eq!(opcode, Opcode::LOAD);
        let opcode = Opcode::from("illegal");
        assert_eq!(opcode, Opcode::ILL);
    }
}
