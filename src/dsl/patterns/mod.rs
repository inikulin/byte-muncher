mod parse;

#[derive(Debug, PartialEq)]
pub enum SetPattern {
    Alpha,
    AlphaLower,
    AlphaUpper,
    Digit,
    Whitespace,
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
    StateEnter,
    Byte(u8),
    Sequence(SequencePattern),
    Set(SetPattern),
    InputState(InputStatePattern),
    Condition(String),
    Any,
}
