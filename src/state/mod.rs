mod parse;

use crate::Arm;

#[derive(PartialEq, Debug)]
pub struct State {
    pub name: String,
    pub arms: Vec<Arm>,
}
