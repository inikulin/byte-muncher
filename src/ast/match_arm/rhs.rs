use super::condition_branch::ConditionBranch;
use crate::ast::ActionList;
use syn::parse::{Parse, ParseStream};
use syn::{Result as ParseResult, Token};

#[derive(PartialEq, Debug)]
pub enum MatchArmRhs {
    ActionList(ActionList),
    Condition {
        if_branch: ConditionBranch,
        else_if_branches: Vec<ConditionBranch>,
        else_branch: ActionList,
    },
}

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
            else_branch: ConditionBranch::parse_braced_action_list(input)?,
        })
    }
}

impl Parse for MatchArmRhs {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        if parse_if_present!(input, { if }) {
            Self::parse_condition(input)
        } else {
            let action_list = input.parse::<ActionList>()?;

            Ok(MatchArmRhs::ActionList(action_list))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::match_arm::condition_branch::ERR_UNEXPECTED_CONTENT_AFTER_ACTION_LIST;

    curry_parse_macros!($MatchArmRhs);

    #[test]
    fn parse_action_list() {
        assert_eq!(
            parse_ok! { ( foo; bar; ) },
            MatchArmRhs::ActionList(ActionList {
                actions: vec![act!("foo"), act!("bar")],
                state_transition: None
            })
        );
    }

    #[test]
    fn parse_simple_condition() {
        assert_eq!(
            parse_ok! [
                if cond {
                    ( foo; bar; )
                } else {
                    ( baz; )
                }
            ],
            MatchArmRhs::Condition {
                if_branch: ConditionBranch {
                    condition: "cond".into(),
                    actions: ActionList {
                        actions: vec![act!("foo"), act!("bar")],
                        state_transition: None
                    }
                },
                else_if_branches: vec![],
                else_branch: ActionList {
                    actions: vec![act!("baz")],
                    state_transition: None
                }
            }
        );
    }

    #[test]
    fn parse_else_if_condition() {
        assert_eq!(
            parse_ok! [
                if cond1 {
                    ( foo; )
                } else if cond2 {
                    ( baz; )
                } else if cond3 {
                    ( qux; )
                } else {
                    ( quz; )
                }
            ],
            MatchArmRhs::Condition {
                if_branch: ConditionBranch {
                    condition: "cond1".into(),
                    actions: ActionList {
                        actions: vec![act!("foo")],
                        state_transition: None
                    }
                },
                else_if_branches: vec![
                    ConditionBranch {
                        condition: "cond2".into(),
                        actions: ActionList {
                            actions: vec![act!("baz")],
                            state_transition: None
                        }
                    },
                    ConditionBranch {
                        condition: "cond3".into(),
                        actions: ActionList {
                            actions: vec![act!("qux")],
                            state_transition: None
                        }
                    }
                ],
                else_branch: ActionList {
                    actions: vec![act!("quz")],
                    state_transition: None
                }
            }
        );
    }

    #[test]
    fn missing_else_in_condition_error() {
        assert_eq!(
            parse_err! [
                if cond {
                    ( foo; )
                }
            ],
            "unexpected end of input, expected `else`"
        );
    }

    #[test]
    fn unexpected_content_after_action_list_in_else_branch_error() {
        assert_eq!(
            parse_err! [
                if cond {
                    ( foo; )
                } else {
                    ( bar; ) 42
                }
            ],
            ERR_UNEXPECTED_CONTENT_AFTER_ACTION_LIST
        );
    }

}
