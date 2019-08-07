use super::*;
use syn::parse::{Parse, ParseStream};
use syn::{Result as ParseResult, Token};

impl MatchArmRhs {
    fn parse_condition(input: ParseStream) -> ParseResult<Self> {
        let if_branch = input.parse::<ConditionBranch>()?;
        let mut else_if_branches = vec![];

        loop {
            input.parse::<Token! { else }>()?;

            if parse_if_present!(input, { if }) {
                else_if_branches.push(input.parse::<ConditionBranch>()?);
            } else {
                break;
            }
        }

        Ok(MatchArmRhs::Condition {
            if_branch,
            else_if_branches,
            else_branch: ConditionBranch::parse_braced_directives(input)?,
        })
    }
}

impl Parse for MatchArmRhs {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        if parse_if_present!(input, { if }) {
            Self::parse_condition(input)
        } else {
            let directives = input.parse::<Directives>()?;

            Ok(MatchArmRhs::Directives(directives))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::condition_branch::ERR_UNEXPECTED_CONTENT_AFTER_DIRECTIVES;
    use super::*;

    curry_parse_macros!($MatchArmRhs);

    #[test]
    fn parse_directives() {
        assert_eq!(
            parse_ok! { foo, bar. },
            MatchArmRhs::Directives(Directives {
                action_calls: vec![act!("foo"), act!("bar")],
                state_transition: None
            })
        );
    }

    #[test]
    fn parse_simple_condition() {
        assert_eq!(
            parse_ok! {
                if cond {
                    foo, bar.
                } else {
                    baz.
                }
            },
            MatchArmRhs::Condition {
                if_branch: ConditionBranch {
                    condition: "cond".into(),
                    directives: Directives {
                        action_calls: vec![act!("foo"), act!("bar")],
                        state_transition: None
                    }
                },
                else_if_branches: vec![],
                else_branch: Directives {
                    action_calls: vec![act!("baz")],
                    state_transition: None
                }
            }
        );
    }

    #[test]
    fn parse_else_if_condition() {
        assert_eq!(
            parse_ok! {
                if cond1 {
                    foo.
                } else if cond2 {
                    baz.
                } else if cond3 {
                    qux.
                } else {
                    quz.
                }
            },
            MatchArmRhs::Condition {
                if_branch: ConditionBranch {
                    condition: "cond1".into(),
                    directives: Directives {
                        action_calls: vec![act!("foo")],
                        state_transition: None
                    }
                },
                else_if_branches: vec![
                    ConditionBranch {
                        condition: "cond2".into(),
                        directives: Directives {
                            action_calls: vec![act!("baz")],
                            state_transition: None
                        }
                    },
                    ConditionBranch {
                        condition: "cond3".into(),
                        directives: Directives {
                            action_calls: vec![act!("qux")],
                            state_transition: None
                        }
                    }
                ],
                else_branch: Directives {
                    action_calls: vec![act!("quz")],
                    state_transition: None
                }
            }
        );
    }

    #[test]
    fn missing_else_in_condition_error() {
        assert_eq!(
            parse_err! {
                if cond {
                    foo.
                }
            },
            "unexpected end of input, expected `else`"
        );
    }

    #[test]
    fn unexpected_content_after_directives_in_else_branch_error() {
        assert_eq!(
            parse_err! {
                if cond {
                    foo.
                } else {
                    bar. 42
                }
            },
            ERR_UNEXPECTED_CONTENT_AFTER_DIRECTIVES
        );
    }

}
