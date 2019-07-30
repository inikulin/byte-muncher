use crate::ast::Directives;
use syn::parse::{Parse, ParseStream};
use syn::{braced, Ident, Result as ParseResult};

pub(super) const ERR_UNEXPECTED_CONTENT_AFTER_DIRECTIVES: &str =
    "condition branch shouldn't contain anything besides a single directive list";

#[derive(PartialEq, Debug)]
pub struct ConditionBranch {
    pub condition: String,
    pub directives: Directives,
}

impl ConditionBranch {
    pub fn parse_braced_directives(input: ParseStream) -> ParseResult<Directives> {
        let braces_content;

        braced!(braces_content in input);

        let actions = braces_content.parse::<Directives>()?;

        if braces_content.is_empty() {
            Ok(actions)
        } else {
            Err(braces_content.error(ERR_UNEXPECTED_CONTENT_AFTER_DIRECTIVES))
        }
    }
}

impl Parse for ConditionBranch {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(ConditionBranch {
            condition: input.parse::<Ident>()?.to_string(),
            directives: Self::parse_braced_directives(input)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ActionCall, CallInfo};

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
                directives: Directives {
                    action_calls: vec![ActionCall {
                        name: "foo".into(),
                        call_info: CallInfo::default()
                    }],
                    state_transition: Some("bar_state".into())
                }
            }
        );
    }

    #[test]
    fn content_after_directives_error() {
        assert_eq!(
            parse_err! [
                some_condition {
                    ( foo; bar; ) ( baz; )
                }
            ],
            ERR_UNEXPECTED_CONTENT_AFTER_DIRECTIVES
        );
    }
}
