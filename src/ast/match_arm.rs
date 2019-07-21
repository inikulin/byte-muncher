use super::action_list::ActionList;
use super::patterns::Pattern;
use syn::parse::{Parse, ParseStream};
use syn::{Result as ParseResult, Token};

#[derive(PartialEq, Debug)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub action_list: ActionList,
}

impl Parse for MatchArm {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let pattern = input.parse::<Pattern>()?;

        input.parse::<Token! { => }>()?;

        Ok(MatchArm {
            pattern,
            action_list: input.parse::<ActionList>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::action_list::StateTransition;
    use crate::ast::patterns::SetPattern;

    curry_parse_macros!($MatchArm);

    #[test]
    fn parse_match_arm() {
        assert_eq!(
            parse_ok! { 'a' => ( foo; --> baz_state ) },
            MatchArm {
                pattern: Pattern::Byte(b'a'),
                action_list: ActionList {
                    actions: vec!["foo".into()],
                    state_transition: Some(StateTransition {
                        to_state: "baz_state".into(),
                        reconsume: false
                    })
                }
            }
        );

        assert_eq!(
            parse_ok! { alpha => ( foo; bar; baz; ) },
            MatchArm {
                pattern: Pattern::Set(SetPattern::Alpha),
                action_list: ActionList {
                    actions: vec!["foo".into(), "bar".into(), "baz".into()],
                    state_transition: None
                }
            }
        );
    }

    #[test]
    fn invalid_connector_token_error() {
        assert_eq!(parse_err! { "foo"|i =< ( bar; ) }, "expected `=>`");
    }
}
