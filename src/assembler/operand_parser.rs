use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::preceded,
    IResult, Parser,
};

use super::Token;

pub fn parse_integer_operand(input: &str) -> IResult<&str, Token> {
    map(
        preceded(
            tag(" #"),
            map_res(digit1, |integer_str: &str| integer_str.parse::<i32>()),
        ),
        |value| Token::IntegerOperand { value },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        let result = parse_integer_operand(" #52");
        assert!(result.is_ok());
        let (left, value) = result.unwrap();
        assert_eq!(left, "");
        assert_eq!(value, Token::IntegerOperand { value: 52 });

        let result = parse_integer_operand("52");
        assert!(result.is_err());

        let result = parse_integer_operand("#z");
        assert!(result.is_err());
    }
}
