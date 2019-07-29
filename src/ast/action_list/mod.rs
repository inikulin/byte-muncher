mod call_info;

use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, Ident, Result as ParseResult, Token};

pub use self::call_info::CallInfo;

const ERR_UNEXPECTED_ITEM: &str = concat![
    "action list should contain semicolon-terminated actions with ",
    "an optional trailing state transition (`--> {state}` or `reconsume in {state}`)"
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
    fn parse_state_transition_target(
        &mut self,
        input: ParseStream,
        reconsume: bool,
    ) -> ParseResult<()> {
        self.state_transition = Some(StateTransition {
            to_state: input.parse::<Ident>()?.to_string(),
            reconsume,
        });

        Ok(())
    }

    fn parse_item(&mut self, input: ParseStream) -> ParseResult<()> {
        if parse3_if_present!(input, { - }, { - }, { > }) {
            self.parse_state_transition_target(input, false)?;
        } else if input.peek(Ident) {
            let action = input.parse::<Ident>()?.to_string();

            if action == "reconsume" && parse_if_present!(input, { in }) {
                self.parse_state_transition_target(input, true)?;
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

        while !items.is_empty() {
            if list.state_transition.is_some() {
                let msg = if items.peek(Token! { ; }) {
                    ERR_SEMICOLON_TERMINATED_TRANSITION
                } else {
                    ERR_TRANSITION_IS_NOT_LAST_ENTRY
                };

                return Err(input.error(msg));
            }

            list.parse_item(&items)?;
        }

        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    curry_parse_macros!($ActionList);

    #[test]
    fn parse_empty_list() {
        assert_eq!(
            parse_ok! { () },
            ActionList {
                actions: vec![],
                state_transition: None
            }
        );
    }

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
