mod parse;

use crate::dsl::State;

#[derive(PartialEq, Debug)]
pub struct Grammar {
    pub name: String,
    pub states: Vec<State>,
}
