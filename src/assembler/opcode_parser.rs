use nom::{character::complete::alpha1, combinator::map, IResult, Parser};

use super::{Opcode, Token};

pub fn parse_opcode(input: &str) -> IResult<&str, Token> {
    map(alpha1, |code_str| Token::Op {
        code: Opcode::from(code_str),
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_opcode_load() {
        let result = parse_opcode("load");
        assert!(result.is_ok());
        let (left, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(left, "");
    }

    #[test]
    fn test_parse_opcode_igl() {
        let result = parse_opcode("illegal");
        assert!(result.is_ok());
        let (left, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });
        assert_eq!(left, "");
    }
}
