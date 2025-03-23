use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res, opt};
use nom::sequence::preceded;
use nom::{IResult, Parser};

use super::{MaybeToken, Token};

pub fn register_parser(input: &str) -> IResult<&str, Token> {
    map(
        preceded(
            tag(" $"),
            map_res(digit1, |inner: &str| inner.parse::<u8>()),
        ),
        |index| Token::Register { index },
    )
    .parse(input)
}

pub fn integer_parser(input: &str) -> IResult<&str, Token> {
    map(
        preceded(
            tag(" #"),
            map_res(digit1, |inner: &str| inner.parse::<i32>()),
        ),
        |value| Token::Integer { value },
    )
    .parse(input)
}

pub fn operand_porser(input: &str) -> IResult<&str, Token> {
    alt((register_parser, integer_parser)).parse(input)
}

pub fn oop(input: &str) -> IResult<&str, MaybeToken> {
    opt(operand_porser).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_parser() {
        let result0 = register_parser(" $52");
        assert!(result0.is_ok());

        let result1 = register_parser("52");
        assert!(result1.is_err());

        let result2 = register_parser("$z");
        assert!(result2.is_err());
    }

    #[test]
    fn test_int_operand_parser() {
        let result0 = integer_parser(" #52");
        assert!(result0.is_ok());
        let (left, value) = result0.unwrap();
        assert_eq!(left, "");
        assert_eq!(value, Token::Integer { value: 52 });

        let result1 = integer_parser("52");
        assert!(result1.is_err());

        let result2 = integer_parser("#z");
        assert!(result2.is_err());
    }
}
