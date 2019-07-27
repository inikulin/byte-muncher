mod condition_branch;
mod rhs;

use super::patterns::Pattern;
use syn::parse::{Parse, ParseStream};
use syn::{Result as ParseResult, Token};
use self::rhs::MatchArmRhs;

#[derive(PartialEq, Debug)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub rhs: MatchArmRhs,
}

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
    use crate::ast::action_list::{ActionList, StateTransition};
    use crate::ast::patterns::SetPattern;

    curry_parse_macros!($MatchArm);

    #[test]
    fn parse_match_arm() {
        assert_eq!(
            parse_ok! { 'a' => ( foo; --> baz_state ) },
            MatchArm {
                pattern: Pattern::Byte(b'a'),
                rhs: MatchArmRhs::ActionList(ActionList {
                    actions: vec!["foo".into()],
                    state_transition: Some(StateTransition {
                        to_state: "baz_state".into(),
                        reconsume: false
                    })
                })
            }
        );

        assert_eq!(
            parse_ok! { alpha => ( foo; bar; baz; ) },
            MatchArm {
                pattern: Pattern::Set(SetPattern::Alpha),
                rhs: MatchArmRhs::ActionList(ActionList {
                    actions: vec!["foo".into(), "bar".into(), "baz".into()],
                    state_transition: None
                })
            }
        );
    }

    #[test]
    fn invalid_connector_token_error() {
        assert_eq!(parse_err! { "foo"|i =< ( bar; ) }, "expected `=>`");
    }
}
