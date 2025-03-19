use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::sequence::{preceded, terminated};
use nom::{IResult, Parser};

use super::Token;

pub fn directive_decl_parser(input: &str) -> IResult<&str, Token> {
    map(
        preceded(tag("!"), map(alpha1, |directive_str: &str| directive_str)),
        |name| Token::Directive {
            name: name.to_string(),
        },
    )
    .parse(input)
}

pub fn label_decl_parser(input: &str) -> IResult<&str, Token> {
    map(
        terminated(map(alpha1, |label_str: &str| label_str), tag(": ")),
        |name| Token::LabelDecl {
            name: name.to_string(),
        },
    )
    .parse(input)
}

pub fn label_usage_parser(input: &str) -> IResult<&str, Token> {
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
    fn test_label_decl_parser() {
        let result0 = label_decl_parser("test: halt");
        assert!(result0.is_ok());
        let (left, token) = result0.unwrap();
        assert_eq!(left, "halt");
        assert_eq!(
            token,
            Token::LabelDecl {
                name: "test".to_string()
            }
        );

        let result1 = label_decl_parser("test");
        assert!(result1.is_err());
    }

    #[test]
    fn test_label_usage_parser() {
        let result0 = label_usage_parser(" @test");
        assert!(result0.is_ok());
        let (_, token) = result0.unwrap();
        assert_eq!(
            token,
            Token::LabelUsage {
                name: "test".to_string()
            }
        );

        let result1 = label_usage_parser("test");
        assert!(result1.is_err());
    }
}
