use crate::ast::action_list::ActionList;
use syn::parse::{Parse, ParseStream};
use syn::{braced, Ident, Result as ParseResult};

pub(super) const ERR_UNEXPECTED_CONTENT_AFTER_ACTION_LIST: &str =
    "condition branch shouldn't contatin anything besides a single action list";

#[derive(PartialEq, Debug)]
pub struct ConditionBranch {
    pub condition: String,
    pub actions: ActionList,
}

impl ConditionBranch {
    pub fn parse_braced_action_list(input: ParseStream) -> ParseResult<ActionList> {
        let braces_content;

        braced!(braces_content in input);

        let actions = braces_content.parse::<ActionList>()?;

        if braces_content.is_empty() {
            Ok(actions)
        } else {
            Err(braces_content.error(ERR_UNEXPECTED_CONTENT_AFTER_ACTION_LIST))
        }
    }
}

impl Parse for ConditionBranch {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(ConditionBranch {
            condition: input.parse::<Ident>()?.to_string(),
            actions: Self::parse_braced_action_list(input)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::action_list::StateTransition;

    curry_parse_macros!($ConditionBranch);

    #[test]
    fn parse_match_arm() {
        assert_eq!(
            parse_ok! [
                cond {
                    ( foo; --> bar_state )
                }
            ],
            ConditionBranch {
                condition: "cond".into(),
                actions: ActionList {
                    actions: vec!["foo".into()],
                    state_transition: Some(StateTransition {
                        to_state: "bar_state".into(),
                        reconsume: false
                    })
                }
            }
        );
    }

    #[test]
    fn content_after_action_list_error() {
        assert_eq!(
            parse_err! [
                some_condition {
                    ( foo; bar; ) ( baz; )
                }
            ],
            ERR_UNEXPECTED_CONTENT_AFTER_ACTION_LIST
        );
    }
}
