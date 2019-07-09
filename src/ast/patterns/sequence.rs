use syn::parse::{Parse, ParseStream};
use syn::{Error as ParseError, Ident, LitStr, Result as ParseResult, Token};

const ERR_STR_IS_NOT_ASCII: &str = concat![
    "characters in string sequence pattern should be in the ASCII range.",
    " Use array sequence patterns instead (e.g. ['f', 0x00, 'O'])"
];

const ERR_UNSUPPORTED_FLAG: &str =
    "unsupported sequence flag. Only ignore case flag (`i`) is currently supported";

#[derive(PartialEq, Debug)]
pub struct SequencePattern {
    pub bytes: Vec<u8>,
    pub ignore_case: bool,
}

impl SequencePattern {
    fn parse_ignore_case_flag(input: ParseStream) -> ParseResult<bool> {
        if input.lookahead1().peek(Token! { | }) {
            input.parse::<Token! { | }>()?;

            let flag = input.parse::<Ident>()?;

            if flag.to_string() == "i" {
                Ok(true)
            } else {
                Err(ParseError::new_spanned(flag, ERR_UNSUPPORTED_FLAG))
            }
        } else {
            Ok(false)
        }
    }

    fn parse_from_str_literal(input: ParseStream) -> ParseResult<Self> {
        let lit = input.parse::<LitStr>()?;
        let string = lit.value();

        if string.is_ascii() {
            Ok(SequencePattern {
                bytes: string.into_bytes(),
                ignore_case: SequencePattern::parse_ignore_case_flag(input)?,
            })
        } else {
            Err(ParseError::new_spanned(lit, ERR_STR_IS_NOT_ASCII))
        }
    }
}

impl Parse for SequencePattern {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr) {
            SequencePattern::parse_from_str_literal(input)
        } else {
            Err(lookahead.error())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    curry_parse_macros!($SequencePattern);

    #[test]
    fn parse_str_literal() {
        assert_eq!(
            parse_ok! { "FooBar" },
            SequencePattern {
                bytes: vec![0x46, 0x6f, 0x6f, 0x42, 0x61, 0x72],
                ignore_case: false
            }
        );

        assert_eq!(
            parse_ok! { "QuXQuZ"|i },
            SequencePattern {
                bytes: vec![0x51, 0x75, 0x58, 0x51, 0x75, 0x5a],
                ignore_case: true
            }
        );
    }

    #[test]
    fn parse_non_ascii_char_literal_error() {
        assert_eq!(parse_err! { "Foo™Bar" }, ERR_STR_IS_NOT_ASCII);
        assert_eq!(parse_err! { "💖 yo 💖" }, ERR_STR_IS_NOT_ASCII);
    }

    #[test]
    fn malformed_flag_error() {
        assert_eq!(parse_err! { "Foo"|"Bar" }, "expected identifier");
    }

    #[test]
    fn unsupported_flag_error() {
        assert_eq!(parse_err! { "Foo"|s }, ERR_UNSUPPORTED_FLAG);
    }
}
