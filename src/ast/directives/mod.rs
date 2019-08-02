mod action_call;

use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result as ParseResult, Token};

pub use self::action_call::ActionCall;

const ERR_UNEXPECTED_ITEM: &str = concat![
    "match arm directives should consist of zero or more colon-terminated action calls with ",
    "an optional trailing state transition (`--> {state}`)"
];

#[derive(Default, PartialEq, Debug)]
pub struct Directives {
    pub action_calls: Vec<ActionCall>,
    pub state_transition: Option<String>,
}

impl Directives {
    fn parse_action_call_terminator(input: ParseStream) -> ParseResult<bool> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token! { , }) {
            input.parse::<Token! { , }>()?;
            Ok(false)
        } else if lookahead.peek(Token! { . }) {
            input.parse::<Token! { . }>()?;
            Ok(true)
        } else {
            Err(lookahead.error())
        }
    }

    fn parse_item(&mut self, input: ParseStream) -> ParseResult<bool> {
        if parse3_if_present!(input, { - }, { - }, { > }) {
            let state_transition = input.parse::<Ident>()?.to_string();

            self.state_transition = Some(state_transition);
            input.parse::<Token! { . }>()?;

            Ok(true)
        } else if input.peek(Ident) {
            self.action_calls.push(input.parse::<ActionCall>()?);
            Self::parse_action_call_terminator(input)
        } else {
            Err(input.error(ERR_UNEXPECTED_ITEM))
        }
    }
}

impl Parse for Directives {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut directives = Self::default();

        // NOTE: check for empty list case before parsing directives
        if !parse_if_present!(input, { . }) {
            while !directives.parse_item(input)? {}
        }

        Ok(directives)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    curry_parse_macros!($Directives);

    #[test]
    fn parse_empty_list() {
        assert_eq!(
            parse_ok! { . },
            Directives {
                action_calls: vec![],
                state_transition: None
            }
        );
    }

    #[test]
    fn parse_action_calls() {
        assert_eq!(
            parse_ok! { foo, bar, baz. },
            Directives {
                action_calls: vec![act!("foo"), act!("bar"), act!("baz")],
                state_transition: None
            }
        );
    }

    #[test]
    fn parse_state_transition() {
        assert_eq!(
            parse_ok! { foo, bar, --> baz_state. },
            Directives {
                action_calls: vec![act!("foo"), act!("bar")],
                state_transition: Some("baz_state".into())
            }
        );

        assert_eq!(
            parse_ok! { --> foo_state. },
            Directives {
                action_calls: vec![],
                state_transition: Some("foo_state".into())
            }
        );
    }

    #[test]
    fn unexpected_item_error() {
        assert_eq!(parse_err! { foo, 123. }, ERR_UNEXPECTED_ITEM);
        assert_eq!(parse_err! { foo, -- bar. }, ERR_UNEXPECTED_ITEM);
    }

    #[test]
    fn unterminated_action_error() {
        assert_eq!(parse_err! { foo, baz bar. }, "expected `,` or `.`");
    }

    #[test]
    fn transition_without_destination_state_error() {
        assert_eq!(
            parse_err! { --> },
            "unexpected end of input, expected identifier"
        );
    }

    #[test]
    fn state_transition_is_not_last_entry_error() {
        assert_eq!(
            parse_err! { foo, --> bar_state, baz. },
            format!("expected `.`")
        );
    }
}
