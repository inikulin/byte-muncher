use super::*;
use syn::parse::{Parse, ParseStream};
use syn::{braced, Ident, Result as ParseResult, Token};

impl Parse for Grammar {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let braces_content;
        let mut states = vec![];
        let name = input.parse::<Ident>()?.to_string();

        input.parse::<Token! { = }>()?;

        braced!(braces_content in input);

        loop {
            states.push(braces_content.parse::<State>()?);

            if braces_content.is_empty() {
                break;
            }
        }

        Ok(Grammar { name, states })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Arm, ArmRhs, Directives, InputStatePattern, Pattern, StateTransition};

    curry_parse_macros!($Grammar);

    #[test]
    fn parse() {
        assert_eq!(
            parse_ok! {
                TestGrammar = {
                    foo_state:
                        'a' => bar, --> baz_state.
                        _   => qux, quz, move --> qux_state.

                    baz_state:
                        eof => qux.
                        _ => --> qux_state.

                    qux_state:
                        --> => qux, quz.
                        _ => quz.
                }
            },
            Grammar {
                name: "TestGrammar".into(),
                states: vec![
                    State {
                        name: "foo_state".into(),
                        arms: vec![
                            Arm {
                                pattern: Pattern::Byte(b'a'),
                                rhs: ArmRhs::Directives(Directives {
                                    action_calls: vec![act!("bar")],
                                    state_transition: Some(StateTransition {
                                        to_state: "baz_state".into(),
                                        epsilon_move: false
                                    })
                                })
                            },
                            Arm {
                                pattern: Pattern::Any,
                                rhs: ArmRhs::Directives(Directives {
                                    action_calls: vec![act!("qux"), act!("quz")],
                                    state_transition: Some(StateTransition {
                                        to_state: "qux_state".into(),
                                        epsilon_move: true
                                    })
                                })
                            }
                        ]
                    },
                    State {
                        name: "baz_state".into(),
                        arms: vec![
                            Arm {
                                pattern: Pattern::InputState(InputStatePattern::Eof),
                                rhs: ArmRhs::Directives(Directives {
                                    action_calls: vec![act!("qux")],
                                    state_transition: None
                                })
                            },
                            Arm {
                                pattern: Pattern::Any,
                                rhs: ArmRhs::Directives(Directives {
                                    action_calls: vec![],
                                    state_transition: Some(StateTransition {
                                        to_state: "qux_state".into(),
                                        epsilon_move: false
                                    })
                                })
                            }
                        ]
                    },
                    State {
                        name: "qux_state".into(),
                        arms: vec![
                            Arm {
                                pattern: Pattern::StateEnter,
                                rhs: ArmRhs::Directives(Directives {
                                    action_calls: vec![act!("qux"), act!("quz")],
                                    state_transition: None
                                })
                            },
                            Arm {
                                pattern: Pattern::Any,
                                rhs: ArmRhs::Directives(Directives {
                                    action_calls: vec![act!("quz")],
                                    state_transition: None
                                })
                            }
                        ]
                    }
                ]
            }
        );
    }

    #[test]
    fn empty_grammar_error() {
        assert_eq!(
            parse_err![FooGrammar = {}],
            "unexpected end of input, expected identifier"
        );
    }
}
