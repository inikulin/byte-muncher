use crate::ast::{ActionList, MatchArm};
use syn::parse::{Parse, ParseStream};
use syn::{braced, Ident, Result as ParseResult};

pub struct State {
    pub name: String,
    pub actions_on_enter: Option<ActionList>,
    pub arms: Vec<MatchArm>,
}

impl State {
    fn parse_actions_on_enter(input: ParseStream) -> ParseResult<Option<ActionList>> {
        if parse3_if_present!(input, { < }, { - }, { - }) {
            input.parse::<ActionList>().map(|l| Some(l))
        } else {
            Ok(None)
        }
    }

    fn parse_arms(input: ParseStream) -> ParseResult<Vec<MatchArm>> {
        let braces_content;
        let mut arms = vec![];

        braced!(braces_content in input);

        loop {
            arms.push(braces_content.parse::<MatchArm>()?);

            if braces_content.is_empty() {
                break;
            }
        }

        Ok(arms)
    }
}

impl Parse for State {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(State {
            name: input.parse::<Ident>()?.to_string(),
            actions_on_enter: Self::parse_actions_on_enter(input)?,
            arms: Self::parse_arms(input)?,
        })
    }
}
