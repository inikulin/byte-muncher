mod call_info;

use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, Ident, Result as ParseResult, Token};

pub use self::call_info::CallInfo;

const ERR_UNEXPECTED_ITEM: &str = concat![
    "match arm directives should consist of zero or more semicolon-terminated action_calls with ",
    "an optional trailing state transition (`--> {state}`)"
];

const ERR_TRANSITION_IS_NOT_LAST_ENTRY: &str =
    "state transition should be the last directive in a match arm";

const ERR_SEMICOLON_TERMINATED_TRANSITION: &str =
    "state transition don't need to be terminated by a semicolon";

#[derive(PartialEq, Debug)]
pub struct ActionCall {
    pub name: String,
    pub call_info: CallInfo,
}

#[derive(Default, PartialEq, Debug)]
pub struct Directives {
    pub action_calls: Vec<ActionCall>,
    pub state_transition: Option<String>,
}

impl Directives {
    fn parse_item(&mut self, input: ParseStream) -> ParseResult<()> {
        if parse3_if_present!(input, { - }, { - }, { > }) {
            let state_transition = input.parse::<Ident>()?.to_string();

            self.state_transition = Some(state_transition);
        } else if input.peek(Ident) {
            let action = input.parse::<Ident>()?.to_string();

            self.action_calls.push(ActionCall {
                name: action,
                call_info: input.parse()?,
            });

            input.parse::<Token! { ; }>()?;
        } else {
            return Err(input.error(ERR_UNEXPECTED_ITEM));
        }

        Ok(())
    }
}

impl Parse for Directives {
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

    curry_parse_macros!($Directives);

    #[test]
    fn parse_empty_list() {
        assert_eq!(
            parse_ok! { () },
            Directives {
                action_calls: vec![],
                state_transition: None
            }
        );
    }

    #[test]
    fn parse_action_calls() {
        assert_eq!(
            parse_ok! { (foo; bar; baz;) },
            Directives {
                action_calls: vec![act!("foo"), act!("bar"), act!("baz")],
                state_transition: None
            }
        );
    }

    #[test]
    fn parse_state_transition() {
        assert_eq!(
            parse_ok! { ( foo; bar; --> baz_state ) },
            Directives {
                action_calls: vec![act!("foo"), act!("bar")],
                state_transition: Some("baz_state".into())
            }
        );

        assert_eq!(
            parse_ok! { ( --> foo_state ) },
            Directives {
                action_calls: vec![],
                state_transition: Some("foo_state".into())
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
    }

    #[test]
    fn unexpected_token_error() {
        assert_eq!(parse_err! { [foo;] }, "expected parentheses");
    }
}
