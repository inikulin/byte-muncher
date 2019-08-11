mod parse;

use crate::State;

#[derive(PartialEq, Debug)]
pub struct Grammar {
    pub name: String,
    pub states: Vec<State>,
}
