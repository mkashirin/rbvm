use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::sequence::preceded;
use nom::{IResult, Parser};

use super::Token;

pub fn register_parser(input: &str) -> IResult<&str, Token> {
    map(
        preceded(
            tag(" $"),
            map_res(digit1, |index_str: &str| index_str.parse::<u8>()),
        ),
        |reg_index| Token::Register { reg_index },
    )
    .parse(input)
}

pub fn int_operand_parser(input: &str) -> IResult<&str, Token> {
    map(
        preceded(
            tag(" #"),
            map_res(digit1, |int_str: &str| int_str.parse::<i32>()),
        ),
        |value| Token::IntOperand { value },
    )
    .parse(input)
}

pub fn operand_porser(input: &str) -> IResult<&str, Token> {
    alt((register_parser, int_operand_parser)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_parser() {
        let result = register_parser(" $52");
        assert!(result.is_ok());

        let result = register_parser("52");
        assert!(result.is_err());

        let result = register_parser("$z");
        assert!(result.is_err());
    }

    #[test]
    fn test_int_operand_parser() {
        let result = int_operand_parser(" #52");
        assert!(result.is_ok());
        let (left, value) = result.unwrap();
        assert_eq!(left, "");
        assert_eq!(value, Token::IntOperand { value: 52 });

        let result = int_operand_parser("52");
        assert!(result.is_err());

        let result = int_operand_parser("#z");
        assert!(result.is_err());
    }
}
