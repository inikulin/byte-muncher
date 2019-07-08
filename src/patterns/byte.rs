use syn::parse::{Parse, ParseStream};
use syn::{Error as ParseError, LitChar, Result as ParseResult};

const ERR_CHAR_IS_NOT_ASCII: &str = "character pattern should be an ASCII character";

#[derive(PartialEq, Debug)]
pub struct BytePattern(u8);

impl BytePattern {
    fn parse_from_char_literal(input: ParseStream) -> ParseResult<Self> {
        let lit_ch = input.parse::<LitChar>()?;
        let ch = lit_ch.value();

        if ch.is_ascii() {
            Ok(BytePattern(ch as u8))
        } else {
            Err(ParseError::new_spanned(lit_ch, ERR_CHAR_IS_NOT_ASCII))
        }
    }
}

impl Parse for BytePattern {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitChar) {
            BytePattern::parse_from_char_literal(input)
        } else {
            Err(lookahead.error())
        }
    }
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    curry_parse_macros!(BytePattern);

    #[test]
    fn parse_char_literal() {
        assert_eq!(parse_ok! { 'a' }, BytePattern(0x61));
        assert_eq!(parse_ok! { '>' }, BytePattern(0x3e));
        assert_eq!(parse_ok! { '0' }, BytePattern(0x30));
    }

    #[test]
    fn parse_char_literal_range_error() {
        assert_eq!(parse_err! { '£' }, ERR_CHAR_IS_NOT_ASCII);
        assert_eq!(parse_err! { '🐼' }, ERR_CHAR_IS_NOT_ASCII);
    }
}
