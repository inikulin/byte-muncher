mod action_call;

use super::*;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result as ParseResult, Token};

#[derive(PartialEq)]
enum Terminator {
    Comma,
    Dot,
}

fn try_parse_state_transition(input: ParseStream) -> ParseResult<Option<StateTransition>> {
    let mut transition = None;
    let epsilon_move = parse_if_present!(input, { move });

    if epsilon_move || input.peek(Token! { - }) {
        parse3!(input, { - }, { - }, { > });

        transition = Some(StateTransition {
            to_state: input.parse::<Ident>()?.to_string(),
            epsilon_move,
        });

        input.parse::<Token! { . }>()?;
    }

    Ok(transition)
}

fn parse_action_call(input: ParseStream) -> ParseResult<(ActionCall, Terminator)> {
    let call = input.parse::<ActionCall>()?;
    let lookahead = input.lookahead1();

    if lookahead.peek(Token! { , }) {
        input.parse::<Token! { , }>()?;
        Ok((call, Terminator::Comma))
    } else if lookahead.peek(Token! { . }) {
        input.parse::<Token! { . }>()?;
        Ok((call, Terminator::Dot))
    } else {
        Err(lookahead.error())
    }
}

impl Parse for Directives {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut directives = Self::default();

        // NOTE: check for empty list case before parsing directives
        if parse_if_present!(input, { . }) {
            return Ok(directives);
        }

        loop {
            directives.state_transition = try_parse_state_transition(input)?;

            if directives.state_transition.is_some() {
                break;
            }

            let (call, terminator) = parse_action_call(input)?;

            directives.action_calls.push(call);

            if terminator == Terminator::Dot {
                break;
            }
        }

        Ok(directives)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StateTransition;

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
                state_transition: Some(StateTransition {
                    to_state: "baz_state".into(),
                    epsilon_move: false
                })
            }
        );

        assert_eq!(
            parse_ok! { --> foo_state. },
            Directives {
                action_calls: vec![],
                state_transition: Some(StateTransition {
                    to_state: "foo_state".into(),
                    epsilon_move: false
                })
            }
        );

        assert_eq!(
            parse_ok! { foo, bar, move --> baz_state. },
            Directives {
                action_calls: vec![act!("foo"), act!("bar")],
                state_transition: Some(StateTransition {
                    to_state: "baz_state".into(),
                    epsilon_move: true
                })
            }
        );

        assert_eq!(
            parse_ok! { move --> foo_state. },
            Directives {
                action_calls: vec![],
                state_transition: Some(StateTransition {
                    to_state: "foo_state".into(),
                    epsilon_move: true
                })
            }
        );
    }

    #[test]
    fn unexpected_item_error() {
        assert_eq!(parse_err! { foo, 123. }, "expected identifier");
        assert_eq!(parse_err! { foo, -- bar. }, "expected `>`");
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
