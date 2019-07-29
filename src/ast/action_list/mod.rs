mod call_info;

use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, Ident, Result as ParseResult, Token};

pub use self::call_info::CallInfo;

const ERR_UNEXPECTED_ITEM: &str = concat![
    "action list should contain a state transition or semicolon-terminated ",
    "action calls with an optional trailing state transition"
];

const ERR_TRANSITION_IS_NOT_LAST_ENTRY: &str =
    "state transition should be the last entry in the action list";

const ERR_SEMICOLON_TERMINATED_TRANSITION: &str =
    "state transition don't need to be terminated by a semicolon";

#[derive(PartialEq, Debug)]
pub struct StateTransition {
    pub to_state: String,
    pub reconsume: bool,
}

impl StateTransition {
    #[inline]
    fn is_arrow(input: ParseStream) -> bool {
        input.peek(Token! { - }) && input.peek2(Token! { - }) && input.peek3(Token! { > })
    }

    #[inline]
    fn parse_dest_state(input: ParseStream, reconsume: bool) -> ParseResult<Self> {
        Ok(StateTransition {
            to_state: input.parse::<Ident>()?.to_string(),
            reconsume,
        })
    }

    fn parse(input: ParseStream) -> ParseResult<Self> {
        input.parse::<Token! { - }>()?;
        input.parse::<Token! { - }>()?;
        input.parse::<Token! { > }>()?;
        Self::parse_dest_state(input, false)
    }

    fn parse_reconsume(input: ParseStream) -> ParseResult<Self> {
        // NOTE: it is expected that "reconsume" identifier is already parsed at this point.
        input.parse::<Token! { in }>()?;
        Self::parse_dest_state(input, true)
    }
}

#[derive(PartialEq, Debug)]
pub struct ActionCall {
    pub name: String,
    pub call_info: CallInfo,
}

#[derive(Default, PartialEq, Debug)]
pub struct ActionList {
    pub actions: Vec<ActionCall>,
    pub state_transition: Option<StateTransition>,
}

impl ActionList {
    fn parse_item(&mut self, input: ParseStream) -> ParseResult<()> {
        if StateTransition::is_arrow(input) {
            self.state_transition = Some(StateTransition::parse(&input)?);
        } else if input.peek(Ident) {
            let action = input.parse::<Ident>()?.to_string();

            if action == "reconsume" && input.peek(Token! { in }) {
                self.state_transition = Some(StateTransition::parse_reconsume(&input)?);
            } else {
                self.actions.push(ActionCall {
                    name: action,
                    call_info: input.parse()?,
                });

                input.parse::<Token! { ; }>()?;
            }
        } else {
            return Err(input.error(ERR_UNEXPECTED_ITEM));
        }

        Ok(())
    }
}

impl Parse for ActionList {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let items;
        let mut list = Self::default();

        parenthesized!(items in input);

        loop {
            list.parse_item(&items)?;

            if items.is_empty() {
                break;
            } else if list.state_transition.is_some() {
                let msg = if items.peek(Token! { ; }) {
                    ERR_SEMICOLON_TERMINATED_TRANSITION
                } else {
                    ERR_TRANSITION_IS_NOT_LAST_ENTRY
                };

                return Err(input.error(msg));
            }
        }

        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    curry_parse_macros!($ActionList);

    #[test]
    fn parse_actions() {
        assert_eq!(
            parse_ok! { (foo; bar; baz;) },
            ActionList {
                actions: vec![act!("foo"), act!("bar"), act!("baz")],
                state_transition: None
            }
        );
    }

    #[test]
    fn parse_state_transition() {
        assert_eq!(
            parse_ok! { ( foo; bar; --> baz_state ) },
            ActionList {
                actions: vec![act!("foo"), act!("bar")],
                state_transition: Some(StateTransition {
                    to_state: "baz_state".into(),
                    reconsume: false
                })
            }
        );

        assert_eq!(
            parse_ok! { ( foo; reconsume in qux_state ) },
            ActionList {
                actions: vec![act!("foo")],
                state_transition: Some(StateTransition {
                    to_state: "qux_state".into(),
                    reconsume: true
                })
            }
        );

        assert_eq!(
            parse_ok! { ( --> foo_state ) },
            ActionList {
                actions: vec![],
                state_transition: Some(StateTransition {
                    to_state: "foo_state".into(),
                    reconsume: false
                })
            }
        );

        assert_eq!(
            parse_ok! { ( reconsume in foo_state ) },
            ActionList {
                actions: vec![],
                state_transition: Some(StateTransition {
                    to_state: "foo_state".into(),
                    reconsume: true
                })
            }
        );
    }

    #[test]
    fn empty_list_error() {
        assert_eq!(
            parse_err! { () },
            format!("unexpected end of input, {}", ERR_UNEXPECTED_ITEM)
        );
    }

    #[test]
    fn unexpected_item_error() {
        assert_eq!(parse_err! { ( foo; 123; ) }, ERR_UNEXPECTED_ITEM);
        assert_eq!(parse_err! { ( foo; -- bar ) }, ERR_UNEXPECTED_ITEM);
    }

    #[test]
    fn unterminated_action_error() {
        assert_eq!(parse_err! { ( foo; baz bar; ) }, "expected `;`");
    }

    #[test]
    fn transition_without_destination_state_error() {
        assert_eq!(
            parse_err! { ( --> ) },
            "unexpected end of input, expected identifier"
        );

        assert_eq!(
            parse_err! { ( reconsume in ) },
            "unexpected end of input, expected identifier"
        );
    }

    #[test]
    fn state_transition_is_not_last_entry_error() {
        assert_eq!(
            parse_err! { ( foo; --> bar_state baz; ) },
            format!(
                "unexpected end of input, {}",
                ERR_TRANSITION_IS_NOT_LAST_ENTRY
            )
        );

        assert_eq!(
            parse_err! { ( foo; --> bar_state --> baz_state; ) },
            format!(
                "unexpected end of input, {}",
                ERR_TRANSITION_IS_NOT_LAST_ENTRY
            )
        );

        assert_eq!(
            parse_err! { ( foo; reconsume in bar_state --> baz_state; ) },
            format!(
                "unexpected end of input, {}",
                ERR_TRANSITION_IS_NOT_LAST_ENTRY
            )
        );
    }

    #[test]
    fn semicolon_terminated_transition_error() {
        assert_eq!(
            parse_err! { ( foo; --> bar_state; ) },
            format!(
                "unexpected end of input, {}",
                ERR_SEMICOLON_TERMINATED_TRANSITION
            )
        );

        assert_eq!(
            parse_err! { ( foo; reconsume in bar_state; ) },
            format!(
                "unexpected end of input, {}",
                ERR_SEMICOLON_TERMINATED_TRANSITION
            )
        );
    }

    #[test]
    fn unexpected_token_error() {
        assert_eq!(parse_err! { [foo;] }, "expected parentheses");
    }
}
