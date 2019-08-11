mod parse;

use crate::dsl::Arm;

#[derive(PartialEq, Debug)]
pub struct State {
    pub name: String,
    pub arms: Vec<Arm>,
}
