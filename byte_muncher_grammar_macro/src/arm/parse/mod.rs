mod condition_branch;
mod patterns;
mod rhs;

use super::*;
use syn::parse::{Parse, ParseStream};
use syn::{Result as ParseResult, Token};

impl Parse for Arm {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let pattern = input.parse::<Pattern>()?;

        input.parse::<Token! { => }>()?;

        Ok(Arm {
            pattern,
            rhs: input.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ActionCall, ClassPattern, ConditionBranch, Directives, StateTransition};

    curry_parse_macros!($Arm);

    #[test]
    fn parse_arm() {
        assert_eq!(
            parse_ok! { 'a' => foo, --> baz_state. },
            Arm {
                pattern: Pattern::Byte(b'a'),
                rhs: ArmRhs::Directives(Directives {
                    action_calls: vec![act!("foo")],
                    state_transition: Some(StateTransition {
                        to_state: "baz_state".into(),
                        epsilon_move: false
                    })
                })
            }
        );

        assert_eq!(
            parse_ok! { alpha => foo, bar, baz(42)?. },
            Arm {
                pattern: Pattern::Class(ClassPattern::Alpha),
                rhs: ArmRhs::Directives(Directives {
                    action_calls: vec![
                        act!("foo"),
                        act!("bar"),
                        ActionCall::UserDefined {
                            name: "baz".into(),
                            args: vec![lit!(42)],
                            with_error_check: true
                        }
                    ],
                    state_transition: None
                })
            }
        );

        assert_eq!(
            parse_ok! {
                'z' => if cond {
                    foo.
                } else {
                    bar.
                }
            },
            Arm {
                pattern: Pattern::Byte(b'z'),
                rhs: ArmRhs::Condition {
                    if_branch: ConditionBranch {
                        condition: "cond".into(),
                        directives: Directives {
                            action_calls: vec![act!("foo")],
                            state_transition: None
                        }
                    },
                    else_if_branches: vec![],
                    else_branch: Directives {
                        action_calls: vec![act!("bar")],
                        state_transition: None
                    }
                }
            }
        );
    }

    #[test]
    fn invalid_connector_token_error() {
        assert_eq!(parse_err! { "foo"|i =< bar. }, "expected `=>`");
    }
}
