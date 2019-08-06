use syn::parse::{Parse, ParseStream};
use syn::token::Bracket;
use syn::{bracketed, Error as ParseError, Ident, LitStr, Result as ParseResult, Token};
use super::*;

const ERR_STR_IS_NOT_ASCII: &str = concat![
    "characters in string sequence pattern should be in the ASCII range.",
    " Use array sequence patterns instead (e.g. ['f', 0x00, 'O'])"
];

const ERR_UNSUPPORTED_FLAG: &str =
    "unsupported sequence flag. Only ignore case flag (`i`) is currently supported";

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
                ignore_case: Self::parse_ignore_case_flag(input)?,
            })
        } else {
            Err(ParseError::new_spanned(lit, ERR_STR_IS_NOT_ASCII))
        }
    }

    fn parse_from_array(input: ParseStream) -> ParseResult<Self> {
        let brackets_content;

        bracketed!(brackets_content in input);

        let bytes = brackets_content
            .parse_terminated::<_, Token! { , }>(BytePattern::parse)?
            .iter()
            .map(|p| p.0)
            .collect();

        Ok(SequencePattern {
            bytes,
            ignore_case: Self::parse_ignore_case_flag(input)?,
        })
    }
}

impl Parse for SequencePattern {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr) {
            Self::parse_from_str_literal(input)
        } else if lookahead.peek(Bracket) {
            Self::parse_from_array(input)
        } else {
            Err(lookahead.error())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::patterns::parse::byte::{ERR_CHAR_IS_NOT_ASCII, ERR_INT_IS_OUT_OF_BOUNDS};

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
    fn parse_array() {
        assert_eq!(
            parse_ok! { [0x46, 'o', 111u64, 0x42, 'a', 0x72] },
            SequencePattern {
                bytes: vec![0x46, 0x6f, 0x6f, 0x42, 0x61, 0x72],
                ignore_case: false
            }
        );

        assert_eq!(
            parse_ok! { [0x51, 'u', 0x58, 0x51, 'u', 0x5a]|i },
            SequencePattern {
                bytes: vec![0x51, 0x75, 0x58, 0x51, 0x75, 0x5a],
                ignore_case: true
            }
        );
    }

    #[test]
    fn non_ascii_char_in_array_error() {
        assert_eq!(parse_err! { ['f', '🐼', 0x51]|i }, ERR_CHAR_IS_NOT_ASCII);
    }

    #[test]
    fn int_literal_in_array_outside_byte_range_error() {
        assert_eq!(
            parse_err! { ['f', 'o', 0x515151] },
            ERR_INT_IS_OUT_OF_BOUNDS
        );
    }

    #[test]
    fn malformed_flag_error() {
        assert_eq!(parse_err! { "Foo"|"Bar" }, "expected identifier");
    }

    #[test]
    fn unsupported_flag_error() {
        assert_eq!(parse_err! { "Foo"|s }, ERR_UNSUPPORTED_FLAG);
    }

    #[test]
    fn unexpected_token_error() {
        assert_eq!(
            parse_err! { -3 },
            "expected string literal or square brackets"
        );
    }
}
