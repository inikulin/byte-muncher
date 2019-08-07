mod condition_branch;
mod rhs;

use syn::parse::{Parse, ParseStream};
use syn::{Result as ParseResult, Token};
use super::*;

impl Parse for MatchArm {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let pattern = input.parse::<Pattern>()?;

        input.parse::<Token! { => }>()?;

        Ok(MatchArm {
            pattern,
            rhs: input.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::{ActionCall, ConditionBranch, Directives, ClassPattern, StateTransition};

    curry_parse_macros!($MatchArm);

    #[test]
    fn parse_match_arm() {
        assert_eq!(
            parse_ok! { 'a' => foo, --> baz_state. },
            MatchArm {
                pattern: Pattern::Byte(b'a'),
                rhs: MatchArmRhs::Directives(Directives {
                    action_calls: vec![act!("foo")],
                    state_transition: Some(StateTransition {
                        to_state: "baz_state".into(),
                        reconsume: false
                    })
                })
            }
        );

        assert_eq!(
            parse_ok! { alpha => foo, bar, baz(42)?. },
            MatchArm {
                pattern: Pattern::Class(ClassPattern::Alpha),
                rhs: MatchArmRhs::Directives(Directives {
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
            parse_ok! [
                'z' => if cond {
                    foo.
                } else {
                    bar.
                }
            ],
            MatchArm {
                pattern: Pattern::Byte(b'z'),
                rhs: MatchArmRhs::Condition {
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
