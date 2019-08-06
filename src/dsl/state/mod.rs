mod parse;

use crate::dsl::MatchArm;

#[derive(PartialEq, Debug)]
pub struct State {
    pub name: String,
    pub arms: Vec<MatchArm>,
}
