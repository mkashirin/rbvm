use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::sequence::{preceded, terminated};
use nom::{IResult, Parser};

use super::Token;

pub fn parse_directive_decl(input: &str) -> IResult<&str, Token> {
    map(
        preceded(tag("!"), map(alpha1, |directive_str: &str| directive_str)),
        |name| Token::Directive {
            name: name.to_string(),
        },
    )
    .parse(input)
}

pub fn parse_label_decl(input: &str) -> IResult<&str, Token> {
    map(
        terminated(map(alpha1, |label_str: &str| label_str), tag(": ")),
        |name| Token::LabelDecl {
            name: name.to_string(),
        },
    )
    .parse(input)
}

pub fn parse_label_usage(input: &str) -> IResult<&str, Token> {
    map(
        preceded(tag(" @"), map(alpha1, |label_str: &str| label_str)),
        |name| Token::LabelUsage {
            name: name.to_string(),
        },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_label_decl() {
        let result0 = parse_label_decl("test: hlt");
        assert!(result0.is_ok());
        let (left, token) = result0.unwrap();
        assert_eq!(left, "hlt");
        assert_eq!(
            token,
            Token::LabelDecl {
                name: "test".to_string()
            }
        );

        let result1 = parse_label_decl("test");
        assert!(result1.is_err());
    }

    #[test]
    fn test_parse_label_usage() {
        let result0 = parse_label_usage(" @test");
        println!("{:?}", result0);
        assert!(result0.is_ok());
        let (left, token) = result0.unwrap();
        assert_eq!(
            token,
            Token::LabelUsage {
                name: "test".to_string()
            }
        );

        let result1 = parse_label_usage("test");
        assert!(result1.is_err());
    }
}
