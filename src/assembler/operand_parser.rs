use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::sequence::preceded;
use nom::{IResult, Parser};

use super::Token;

pub fn parse_register(input: &str) -> IResult<&str, Token> {
    map(
        preceded(
            tag(" $"),
            map_res(digit1, |index_str: &str| index_str.parse::<u8>()),
        ),
        |reg_index| Token::Register { reg_index },
    )
    .parse(input)
}

pub fn parse_int_operand(input: &str) -> IResult<&str, Token> {
    map(
        preceded(
            tag(" #"),
            map_res(digit1, |int_str: &str| int_str.parse::<i32>()),
        ),
        |value| Token::IntOperand { value },
    )
    .parse(input)
}

pub fn parse_operand(input: &str) -> IResult<&str, Token> {
    alt((parse_register, parse_int_operand)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = parse_register(" $52");
        assert!(result.is_ok());

        let result = parse_register("52");
        assert!(result.is_err());

        let result = parse_register("$z");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_integer_operand() {
        let result = parse_int_operand(" #52");
        assert!(result.is_ok());
        let (left, value) = result.unwrap();
        assert_eq!(left, "");
        assert_eq!(value, Token::IntOperand { value: 52 });

        let result = parse_int_operand("52");
        assert!(result.is_err());

        let result = parse_int_operand("#z");
        assert!(result.is_err());
    }
}
