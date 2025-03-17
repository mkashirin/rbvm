use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::preceded,
    IResult, Parser,
};

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
}
