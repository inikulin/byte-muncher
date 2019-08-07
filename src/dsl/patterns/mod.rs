mod parse;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
    Byte(u8),
    Class(ClassPattern),
    Condition(String),
    Any,
    StateEnter,
    Sequence(SequencePattern),
    InputState(InputStatePattern),
}
