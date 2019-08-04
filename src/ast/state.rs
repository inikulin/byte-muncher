use crate::ast::MatchArm;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result as ParseResult, Token};

#[derive(PartialEq, Debug)]
pub struct State {
    pub name: String,
    pub arms: Vec<MatchArm>,
}

impl State {
    #[inline]
    fn is_next_state_name(input: ParseStream) -> bool {
        input.peek(Ident) && input.peek2(Token! { : })
    }

    fn parse_arms(input: ParseStream) -> ParseResult<Vec<MatchArm>> {
        let mut arms = vec![];

        loop {
            arms.push(input.parse::<MatchArm>()?);

            if input.is_empty() || Self::is_next_state_name(input) {
                break;
            }
        }

        Ok(arms)
    }
}

impl Parse for State {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let name = input.parse::<Ident>()?.to_string();

        input.parse::<Token! { : }>()?;

        Ok(State {
            name,
            arms: Self::parse_arms(input)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Directives, MatchArmRhs, Pattern, StateTransition};

    curry_parse_macros!($State);

    #[test]
    fn parse() {
        assert_eq!(
            parse_ok! [
                foo_state:
                    'a' => bar, --> baz_state.
                    _   => qux, quz, as in qux_state.
            ],
            State {
                name: "foo_state".into(),
                arms: vec![
                    MatchArm {
                        pattern: Pattern::Byte(b'a'),
                        rhs: MatchArmRhs::Directives(Directives {
                            action_calls: vec![act!("bar")],
                            state_transition: Some(StateTransition {
                                to_state: "baz_state".into(),
                                reconsume: false
                            })
                        })
                    },
                    MatchArm {
                        pattern: Pattern::Any,
                        rhs: MatchArmRhs::Directives(Directives {
                            action_calls: vec![act!("qux"), act!("quz")],
                            state_transition: Some(StateTransition {
                                to_state: "qux_state".into(),
                                reconsume: true
                            })
                        })
                    }
                ]
            }
        );
    }

    #[test]
    fn no_arms_error() {
        assert_eq!(
            parse_err![
                foo_state:
            ],
            concat![
                "unexpected end of input, expected one of: character literal, integer literal, ",
                "string literal, square brackets, identifier, `_`, `if`"
            ]
        );
    }
}
