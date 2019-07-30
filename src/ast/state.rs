use crate::ast::{Directives, MatchArm};
use syn::parse::{Parse, ParseStream};
use syn::{braced, Ident, Result as ParseResult};

// TODO
// 1. <back> action (only one in list)
// 2. <mark>(ident) action
// 3. range pattern
// 4. Set patter to PredefinedAlias

const ERR_TRANSITION_IN_ENTER_ACTIONS: &str =
    "state enter directives contain a state transition, i.e. state body will never be executed";

#[derive(PartialEq, Debug)]
pub struct State {
    pub name: String,
    pub actions_on_enter: Option<Directives>,
    pub arms: Vec<MatchArm>,
}

impl State {
    fn parse_actions_on_enter(input: ParseStream) -> ParseResult<Option<Directives>> {
        if parse3_if_present!(input, { < }, { - }, { - }) {
            let actions = input.parse::<Directives>()?;

            if actions.state_transition.is_none() {
                Ok(Some(actions))
            } else {
                Err(input.error(ERR_TRANSITION_IN_ENTER_ACTIONS))
            }
        } else {
            Ok(None)
        }
    }

    fn parse_arms(input: ParseStream) -> ParseResult<Vec<MatchArm>> {
        let braces_content;
        let mut arms = vec![];

        braced!(braces_content in input);

        loop {
            arms.push(braces_content.parse::<MatchArm>()?);

            if braces_content.is_empty() {
                break;
            }
        }

        Ok(arms)
    }
}

impl Parse for State {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(State {
            name: input.parse::<Ident>()?.to_string(),
            actions_on_enter: Self::parse_actions_on_enter(input)?,
            arms: Self::parse_arms(input)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{MatchArmRhs, Pattern};

    curry_parse_macros!($State);

    #[test]
    fn parse_simple() {
        assert_eq!(
            parse_ok! [
                foo_state {
                    'a' => ( bar; --> baz_state )
                    _ => ( qux; quz; )
                }
            ],
            State {
                name: "foo_state".into(),
                actions_on_enter: None,
                arms: vec![
                    MatchArm {
                        pattern: Pattern::Byte(b'a'),
                        rhs: MatchArmRhs::Directives(Directives {
                            action_calls: vec![act!("bar")],
                            state_transition: Some("baz_state".into())
                        })
                    },
                    MatchArm {
                        pattern: Pattern::Any,
                        rhs: MatchArmRhs::Directives(Directives {
                            action_calls: vec![act!("qux"), act!("quz")],
                            state_transition: None
                        })
                    }
                ]
            }
        );
    }

    #[test]
    fn parse_with_actions_on_enter() {
        assert_eq!(
            parse_ok! [
                foo_state <-- ( bar; ) {
                    _ => ( baz; )
                }
            ],
            State {
                name: "foo_state".into(),
                actions_on_enter: Some(Directives {
                    action_calls: vec![act!("bar")],
                    state_transition: None
                }),
                arms: vec![MatchArm {
                    pattern: Pattern::Any,
                    rhs: MatchArmRhs::Directives(Directives {
                        action_calls: vec![act!("baz")],
                        state_transition: None
                    })
                }]
            }
        );
    }

    #[test]
    fn no_arms_error() {
        assert_eq!(
            parse_err![foo_state {}],
            concat![
                "unexpected end of input, expected one of: character literal, integer literal, ",
                "string literal, square brackets, identifier, `_`, `if`"
            ]
        );
    }

    #[test]
    fn state_transition_in_state_enter_directives_error() {
        assert_eq!(
            parse_err![
                foo_state <-- ( --> bar_state) {
                    _ => ( bax; )
                }
            ],
            ERR_TRANSITION_IN_ENTER_ACTIONS
        );
    }
}
