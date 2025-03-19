use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::{IResult, Parser};

use super::{Opcode, Token};

pub fn opcode_parser(input: &str) -> IResult<&str, Token> {
    map(alpha1, |code_str| Token::Op {
        code: Opcode::from(code_str),
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_parser_load() {
        let result = opcode_parser("load");
        assert!(result.is_ok());
        let (left, token) = result.unwrap();
        assert_eq!(left, "");
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
    }

    #[test]
    fn test_opcode_parser_ill() {
        let result = opcode_parser("illegal");
        assert!(result.is_ok());
        let (left, token) = result.unwrap();
        assert_eq!(left, "");
        assert_eq!(token, Token::Op { code: Opcode::ILL });
    }
}
