use super::*;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result as ParseResult, Token};

fn parse_arms(input: ParseStream) -> ParseResult<Vec<Arm>> {
    let mut arms = vec![];

    loop {
        arms.push(input.parse::<Arm>()?);

        let is_next_state_name = input.peek(Ident) && input.peek2(Token! { : });

        if is_next_state_name || input.is_empty() {
            break;
        }
    }

    Ok(arms)
}

impl Parse for State {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let name = input.parse::<Ident>()?.to_string();

        input.parse::<Token! { : }>()?;

        Ok(State {
            name,
            arms: parse_arms(input)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::{ArmRhs, Directives, Pattern, StateTransition};

    curry_parse_macros!($State);

    #[test]
    fn parse() {
        assert_eq!(
            parse_ok! {
                foo_state:
                    'a' => bar, --> baz_state.
                    _   => qux, quz, as in qux_state.
            },
            State {
                name: "foo_state".into(),
                arms: vec![
                    Arm {
                        pattern: Pattern::Byte(b'a'),
                        rhs: ArmRhs::Directives(Directives {
                            action_calls: vec![act!("bar")],
                            state_transition: Some(StateTransition {
                                to_state: "baz_state".into(),
                                reconsume: false
                            })
                        })
                    },
                    Arm {
                        pattern: Pattern::Any,
                        rhs: ArmRhs::Directives(Directives {
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
