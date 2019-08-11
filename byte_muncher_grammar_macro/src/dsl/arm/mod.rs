mod compile;
mod parse;

use crate::dsl::Directives;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ClassPattern {
    Alnum,
    Alpha,
    Ascii,
    Lower,
    Upper,
    Digit,
    Xdigit,
    Space,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum InputStatePattern {
    Eoc,
    Eof,
}

#[derive(PartialEq, Debug)]
pub struct SequencePattern {
    pub bytes: Vec<u8>,
    pub ignore_case: bool,
}

#[derive(Debug, PartialEq)]
pub enum Pattern {
    StateEnter,
    Byte(u8),
    Class(ClassPattern),
    InputState(InputStatePattern),
    Condition(String),
    Sequence(SequencePattern),
    Any,
}

#[derive(PartialEq, Debug)]
pub struct ConditionBranch {
    pub condition: String,
    pub directives: Directives,
}

#[derive(PartialEq, Debug)]
pub enum ArmRhs {
    Directives(Directives),
    Condition {
        if_branch: ConditionBranch,
        else_if_branches: Vec<ConditionBranch>,
        else_branch: Directives,
    },
}

#[derive(PartialEq, Debug)]
pub struct Arm {
    pub pattern: Pattern,
    pub rhs: ArmRhs,
}
