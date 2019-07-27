use syn::parse::{Parse, ParseStream};
use syn::token::Paren;
use syn::{parenthesized, Lit, Result as ParseResult, Token};

const ERR_EMPTY_ARGS: &str = "expected at least one action argument";

#[derive(Debug, PartialEq, Default)]
pub struct CallInfo {
    pub args: Vec<Lit>,
    pub with_error_check: bool,
}

impl CallInfo {
    fn parse_args(input: ParseStream) -> ParseResult<Vec<Lit>> {
        if input.peek(Paren) {
            let parens_content;

            parenthesized!(parens_content in input);

            let args = parens_content
                .parse_terminated::<_, Token! { , }>(Lit::parse)?
                .into_iter()
                .collect::<Vec<_>>();

            if args.is_empty() {
                Err(input.error(ERR_EMPTY_ARGS))
            } else {
                Ok(args)
            }
        } else {
            Ok(vec![])
        }
    }
}

impl Parse for CallInfo {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(CallInfo {
            args: Self::parse_args(input)?,
            with_error_check: if input.peek(Token! { ? }) {
                input.parse::<Token! { ? }>()?;
                true
            } else {
                false
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    curry_parse_macros!($CallInfo);

    #[test]
    fn parse() {
        assert_eq!(
            parse_ok! {},
            CallInfo {
                args: vec![],
                with_error_check: false
            }
        );

        assert_eq!(
            parse_ok! { ("foo", 123) },
            CallInfo {
                args: vec![lit!("foo"), lit!(123)],
                with_error_check: false
            }
        );

        assert_eq!(
            parse_ok! { (true, 123)? },
            CallInfo {
                args: vec![lit!(true), lit!(123)],
                with_error_check: true
            }
        );

        assert_eq!(
            parse_ok! { ? },
            CallInfo {
                args: vec![],
                with_error_check: true
            }
        );
    }

    #[test]
    fn empty_arg_list_error() {
        assert_eq!(parse_err! { ()? }, ERR_EMPTY_ARGS);
    }
}
