use syn::parse::{Parse, ParseStream};
use syn::token::Paren;
use syn::{parenthesized, Ident, Lit, Result as ParseResult, Token};

const ERR_EMPTY_ARGS: &str = concat![
    "expected at least one action argument (action calls ",
    "without arguments don't need to have parentheses)"
];

#[derive(Debug, PartialEq)]
pub enum ActionCall {
    UserDefined {
        name: String,
        args: Vec<Lit>,
        with_error_check: bool,
    },
}

impl ActionCall {
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

impl Parse for ActionCall {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(ActionCall::UserDefined {
            name: input.parse::<Ident>()?.to_string(),
            args: Self::parse_args(input)?,
            with_error_check: parse_if_present!(input, { ? }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    curry_parse_macros!($ActionCall);

    #[test]
    fn parse_user_defined() {
        assert_eq!(
            parse_ok! { foo },
            ActionCall::UserDefined {
                name: "foo".into(),
                args: vec![],
                with_error_check: false
            }
        );

        assert_eq!(
            parse_ok! { foo("bar", 123) },
            ActionCall::UserDefined {
                name: "foo".into(),
                args: vec![lit!("bar"), lit!(123)],
                with_error_check: false
            }
        );

        assert_eq!(
            parse_ok! { bar(true, 123)? },
            ActionCall::UserDefined {
                name: "bar".into(),
                args: vec![lit!(true), lit!(123)],
                with_error_check: true
            }
        );

        assert_eq!(
            parse_ok! { baz? },
            ActionCall::UserDefined {
                name: "baz".into(),
                args: vec![],
                with_error_check: true
            }
        );
    }

    #[test]
    fn empty_arg_list_error() {
        assert_eq!(parse_err! { foo()? }, ERR_EMPTY_ARGS);
    }
}
