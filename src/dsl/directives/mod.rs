mod parse;

use syn::Lit;

#[derive(PartialEq, Debug)]
pub struct StateTransition {
    pub to_state: String,
    pub reconsume: bool,
}

#[derive(Default, PartialEq, Debug)]
pub struct Directives {
    pub action_calls: Vec<ActionCall>,
    pub state_transition: Option<StateTransition>,
}

#[derive(Debug, PartialEq)]
pub enum ActionCall {
    UserDefined {
        name: String,
        args: Vec<Lit>,
        with_error_check: bool,
    },
    Start(String),
    End(String),
}
