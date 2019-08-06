mod compile;
mod parse;

#[derive(Debug, PartialEq)]
pub enum AliasPattern {
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
    Byte(u8),
    Alias(AliasPattern),
    Condition(String),
    Any,
    StateEnter,
    Sequence(SequencePattern),
    InputState(InputStatePattern),
}
