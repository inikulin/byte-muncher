mod parse;
mod compile;

use crate::dsl::{Directives, Pattern};

#[derive(PartialEq, Debug)]
pub struct ConditionBranch {
    pub condition: String,
    pub directives: Directives,
}

#[derive(PartialEq, Debug)]
pub enum MatchArmRhs {
    Directives(Directives),
    Condition {
        if_branch: ConditionBranch,
        else_if_branches: Vec<ConditionBranch>,
        else_branch: Directives,
    },
}

#[derive(PartialEq, Debug)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub rhs: MatchArmRhs,
}
