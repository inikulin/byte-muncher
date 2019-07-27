mod byte;
mod sequence;

use syn::parse::{Parse, ParseStream};
use syn::token::Bracket;
use syn::{Error as ParseError, Ident, LitChar, LitInt, LitStr, Result as ParseResult, Token};

pub use self::byte::BytePattern;
pub use self::sequence::SequencePattern;

const ERR_UNKNOWN_PATTERN: &str = "unknown pattern";

#[derive(Debug, PartialEq)]
pub enum SetPattern {
    Alpha,
    AlphaLower,
    AlphaUpper,
    Digit,
    Whitespace,
}

#[derive(Debug, PartialEq)]
pub enum InputStatePattern {
    Eoc,
    Eof,
}

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Byte(u8),
    Sequence(SequencePattern),
    Set(SetPattern),
    InputState(InputStatePattern),
    Condition(String),
    Any,
}

impl Pattern {
    fn parse_from_ident(input: ParseStream) -> ParseResult<Self> {
        macro_rules! set_pat {
            ($Type:ident) => {
                Ok(Pattern::Set(SetPattern::$Type))
            };
        }

        macro_rules! input_state_pat {
            ($Type:ident) => {
                Ok(Pattern::InputState(InputStatePattern::$Type))
            };
        }

        let ident = input.parse::<Ident>()?;

        match ident.to_string().as_str() {
            "alpha" => set_pat!(Alpha),
            "alpha_lo" => set_pat!(AlphaLower),
            "alpha_up" => set_pat!(AlphaUpper),
            "digit" => set_pat!(Digit),
            "ws" => set_pat!(Whitespace),

            "eoc" => input_state_pat!(Eoc),
            "eof" => input_state_pat!(Eof),

            _ => Err(ParseError::new_spanned(ident, ERR_UNKNOWN_PATTERN)),
        }
    }
}

impl Parse for Pattern {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitChar) || lookahead.peek(LitInt) {
            Ok(Pattern::Byte(input.parse::<BytePattern>()?.0))
        } else if lookahead.peek(LitStr) || lookahead.peek(Bracket) {
            Ok(Pattern::Sequence(input.parse::<SequencePattern>()?))
        } else if lookahead.peek(Ident) {
            Ok(Self::parse_from_ident(input)?)
        } else if lookahead.peek(Token! { _ }) {
            input.parse::<Token! { _ }>()?;

            Ok(Pattern::Any)
        } else if lookahead.peek(Token! { if }) {
            input.parse::<Token! { if }>()?;

            Ok(Pattern::Condition(input.parse::<Ident>()?.to_string()))
        } else {
            Err(lookahead.error())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    curry_parse_macros!($Pattern);

    #[test]
    fn parse_byte_pattern() {
        assert_eq!(parse_ok! { 'a' }, Pattern::Byte(0x61));
        assert_eq!(parse_ok! { 0x61 }, Pattern::Byte(0x61));
    }

    #[test]
    fn parse_seq_pattern() {
        assert_eq!(
            parse_ok! { "FooBar"|i },
            Pattern::Sequence(SequencePattern {
                bytes: vec![0x46, 0x6f, 0x6f, 0x42, 0x61, 0x72],
                ignore_case: true
            })
        );

        assert_eq!(
            parse_ok! { [1, 2, 0x03]|i },
            Pattern::Sequence(SequencePattern {
                bytes: vec![0x01, 0x02, 0x03],
                ignore_case: true
            })
        );
    }

    #[test]
    fn parse_set_pattern() {
        assert_eq!(parse_ok! { alpha }, Pattern::Set(SetPattern::Alpha));
        assert_eq!(parse_ok! { alpha_lo }, Pattern::Set(SetPattern::AlphaLower));
        assert_eq!(parse_ok! { alpha_up }, Pattern::Set(SetPattern::AlphaUpper));
        assert_eq!(parse_ok! { digit }, Pattern::Set(SetPattern::Digit));
        assert_eq!(parse_ok! { ws }, Pattern::Set(SetPattern::Whitespace));
    }

    #[test]
    fn parse_input_state_pattern() {
        assert_eq!(
            parse_ok! { eoc },
            Pattern::InputState(InputStatePattern::Eoc)
        );
        assert_eq!(
            parse_ok! { eof },
            Pattern::InputState(InputStatePattern::Eof)
        );
    }

    #[test]
    fn parse_condition_pattern() {
        assert_eq!(parse_ok! { if foobar }, Pattern::Condition("foobar".into()));
    }

    #[test]
    fn condition_pattern_without_identifier_error() {
        assert_eq!(parse_err! { if => }, "expected identifier");
    }

    #[test]
    fn parse_any_pattern() {
        assert_eq!(parse_ok! { _ }, Pattern::Any);
    }

    #[test]
    fn unknown_pattern_error() {
        assert_eq!(parse_err! { foobar }, ERR_UNKNOWN_PATTERN);
    }

    #[test]
    fn unexpected_token_error() {
        assert_eq!(
            parse_err! { -3 },
            concat![
                "expected one of: character literal, integer literal, ",
                "string literal, square brackets, identifier, `_`, `if`"
            ]
        );
    }
}
