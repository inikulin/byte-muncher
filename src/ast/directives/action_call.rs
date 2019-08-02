use syn::parse::{Parse, ParseStream};
use syn::token::Paren;
use syn::{parenthesized, Error as ParseError, Ident, Lit, Result as ParseResult, Token};

const ERR_EMPTY_ARGS: &str = concat![
    "expected at least one action argument (action calls ",
    "without arguments don't need to have parentheses)"
];

const ERR_UNKNOW_BUILT_IN: &str = "unknown built-in action";
const ERR_MARK_TOO_MANY_ARGUMENTS: &str = "<mark> has only one argument";

#[derive(Debug, PartialEq)]
pub enum ActionCall {
    UserDefined {
        name: String,
        args: Vec<Lit>,
        with_error_check: bool,
    },
    Back,
    Mark(String),
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

    fn parse_marker(input: ParseStream) -> ParseResult<String> {
        let parens_content;

        parenthesized!(parens_content in input);

        let marker = parens_content.parse::<Ident>()?.to_string();

        if parens_content.is_empty() {
            Ok(marker)
        } else {
            Err(parens_content.error(ERR_MARK_TOO_MANY_ARGUMENTS))
        }
    }
}

impl Parse for ActionCall {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let built_in = parse_if_present!(input, { < });
        let name_ident = input.parse::<Ident>()?;
        let name = name_ident.to_string();

        let call = if built_in {
            input.parse::<Token! { > }>()?;

            match name.as_str() {
                "back" => ActionCall::Back,
                "mark" => ActionCall::Mark(Self::parse_marker(input)?),
                _ => return Err(ParseError::new_spanned(name_ident, ERR_UNKNOW_BUILT_IN)),
            }
        } else {
            ActionCall::UserDefined {
                name,
                args: Self::parse_args(input)?,
                with_error_check: parse_if_present!(input, { ? }),
            }
        };

        input.parse::<Token! { ; }>()?;

        Ok(call)
    }
}

/*
syntax!(Json = {
    @initial:
        '"' => --> content.
        _   => error?.

    content:
        --> => <mark>(start).
        '"' => --> end.

    end:
        _ => error?.
});


#[derive(Parser)]
pub struct JsonParser(Json::ParsingContext);

impl Json::StateMachine for StringParser {
    fn context(&mut self) -> StringSm::Context {
        self.0
    }

    fn string(&mut self, ) -> Result<String, Error> {
        String::from_utf8(input[ctx.marker.start..ctx.marker.end])
    }
}
*/





#[cfg(test)]
mod tests {
    use super::*;

    curry_parse_macros!($ActionCall);

    #[test]
    fn parse_user_defined() {
        assert_eq!(
            parse_ok! { foo; },
            ActionCall::UserDefined {
                name: "foo".into(),
                args: vec![],
                with_error_check: false
            }
        );

        assert_eq!(
            parse_ok! { foo("bar", 123); },
            ActionCall::UserDefined {
                name: "foo".into(),
                args: vec![lit!("bar"), lit!(123)],
                with_error_check: false
            }
        );

        assert_eq!(
            parse_ok! { bar(true, 123)?; },
            ActionCall::UserDefined {
                name: "bar".into(),
                args: vec![lit!(true), lit!(123)],
                with_error_check: true
            }
        );

        assert_eq!(
            parse_ok! { baz?; },
            ActionCall::UserDefined {
                name: "baz".into(),
                args: vec![],
                with_error_check: true
            }
        );
    }

    #[test]
    fn parse_back_built_in() {
        assert_eq!(parse_ok! { <back>; }, ActionCall::Back);
    }

    #[test]
    fn parse_mark_built_in() {
        assert_eq!(
            parse_ok! { <mark>(token_start); },
            ActionCall::Mark("token_start".into())
        );
    }

    #[test]
    fn missing_mark_built_in_argument_error() {
        assert_eq!(
            parse_err! { <mark>(); },
            "unexpected end of input, expected identifier"
        );
    }

    #[test]
    fn mark_built_in_too_many_arguments_error() {
        assert_eq!(parse_err! { <mark>(m1, m2); }, ERR_MARK_TOO_MANY_ARGUMENTS);
    }

    #[test]
    fn unknown_built_in_error() {
        assert_eq!(parse_err! { <foo>(); }, ERR_UNKNOW_BUILT_IN);
    }

    #[test]
    fn empty_arg_list_error() {
        assert_eq!(parse_err! { foo()?; }, ERR_EMPTY_ARGS);
    }
}
