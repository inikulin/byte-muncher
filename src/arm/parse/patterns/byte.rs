use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Error as ParseError, LitChar, LitInt, Result as ParseResult};

pub(super) const ERR_CHAR_IS_NOT_ASCII: &str = concat![
    "character should be in the ASCII range.",
    " For bigger byte values use numeric representation (e.g. 0x1F)"
];

pub(super) const ERR_INT_IS_OUT_OF_BOUNDS: &str =
    "numeric pattern is not in the byte value range (0x00-0xFF, 0-255, etc.)";

#[derive(PartialEq, Debug)]
pub struct BytePattern(pub u8);

impl BytePattern {
    fn parse_from_char_literal(input: ParseStream) -> ParseResult<Self> {
        let lit = input.parse::<LitChar>()?;
        let ch = lit.value();

        if ch.is_ascii() {
            Ok(BytePattern(ch as u8))
        } else {
            Err(ParseError::new_spanned(lit, ERR_CHAR_IS_NOT_ASCII))
        }
    }

    fn parse_from_int_literal(input: ParseStream) -> ParseResult<Self> {
        let lit = input.parse::<LitInt>()?;

        u8::try_from(lit.value())
            .map(BytePattern)
            .map_err(|_| ParseError::new_spanned(lit, ERR_INT_IS_OUT_OF_BOUNDS))
    }
}

impl Parse for BytePattern {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitChar) {
            Self::parse_from_char_literal(input)
        } else if lookahead.peek(LitInt) {
            Self::parse_from_int_literal(input)
        } else {
            Err(lookahead.error())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    curry_parse_macros!($BytePattern);

    #[test]
    fn parse_char_literal() {
        assert_eq!(parse_ok! { 'a' }, BytePattern(0x61));
        assert_eq!(parse_ok! { '>' }, BytePattern(0x3e));
        assert_eq!(parse_ok! { '0' }, BytePattern(0x30));
    }

    #[test]
    fn non_ascii_char_literal_error() {
        assert_eq!(parse_err! { '£' }, ERR_CHAR_IS_NOT_ASCII);
        assert_eq!(parse_err! { '🐼' }, ERR_CHAR_IS_NOT_ASCII);
    }

    #[test]
    fn parse_int_literal() {
        assert_eq!(parse_ok! { 0x61 }, BytePattern(0x61));
        assert_eq!(parse_ok! { 62 }, BytePattern(0x3e));
        assert_eq!(parse_ok! { 48u64 }, BytePattern(0x30));
    }

    #[test]
    fn int_literal_outside_byte_range_error() {
        assert_eq!(parse_err! { 0x777 }, ERR_INT_IS_OUT_OF_BOUNDS);
        assert_eq!(parse_err! { 256 }, ERR_INT_IS_OUT_OF_BOUNDS);
    }

    #[test]
    fn unexpected_token_error() {
        assert_eq!(
            parse_err! { -3 },
            "expected character literal or integer literal"
        );
    }
}
