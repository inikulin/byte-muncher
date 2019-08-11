use super::*;
use syn::parse::{Parse, ParseStream};
use syn::token::Paren;
use syn::{parenthesized, Error as ParseError, Ident, Lit, Result as ParseResult, Token};

const ERR_EMPTY_ARGS: &str = concat![
    "expected at least one action argument (action calls ",
    "without arguments don't need to have parentheses)"
];

const ERR_TOO_MANY_ARGS: &str = "too many arguments";
const ERR_UNKNOWN_BUILT_IN: &str = "unknown built-in directive";

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

fn parse_pin_name(input: ParseStream) -> ParseResult<String> {
    let parens_content;

    parenthesized!(parens_content in input);

    let name = parens_content.parse::<Ident>()?.to_string();

    if parens_content.is_empty() {
        Ok(name)
    } else {
        Err(parens_content.error(ERR_TOO_MANY_ARGS))
    }
}

impl Parse for ActionCall {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let built_in = parse_if_present!(input, { @ });
        let name_ident = input.parse::<Ident>()?;
        let name = name_ident.to_string();

        if built_in {
            match name.as_str() {
                "start" => parse_pin_name(input).map(ActionCall::Start),
                "end" => parse_pin_name(input).map(ActionCall::End),
                _ => Err(ParseError::new_spanned(name_ident, ERR_UNKNOWN_BUILT_IN)),
            }
        } else {
            Ok(ActionCall::UserDefined {
                name,
                args: parse_args(input)?,
                with_error_check: parse_if_present!(input, { ? }),
            })
        }
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
    fn parse_built_in() {
        assert_eq!(parse_ok! { @start(foo) }, ActionCall::Start("foo".into()));
        assert_eq!(parse_ok! { @end(bar) }, ActionCall::End("bar".into()));
    }

    #[test]
    fn too_many_args_for_built_in_error() {
        assert_eq!(parse_err! { @start(foo, bar) }, ERR_TOO_MANY_ARGS);
        assert_eq!(parse_err! { @end(foo, bar) }, ERR_TOO_MANY_ARGS);
    }

    #[test]
    fn unknown_built_in_error() {
        assert_eq!(parse_err! { @foo }, ERR_UNKNOWN_BUILT_IN);
    }

    #[test]
    fn empty_arg_list_error() {
        assert_eq!(parse_err! { foo()? }, ERR_EMPTY_ARGS);
    }
}
