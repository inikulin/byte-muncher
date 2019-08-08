#![deny(clippy::all)]

mod dsl;

// TODO
// v0.1.0
// 3. Rename `as in <state>` to `move --> <state>` (reconsume to epsilon move)
// 4. Initial state
// 5. Generate SM streaming
// 6. module system
// 7. cool_thing POC

// v0.2.0
// 1. Range pattern
// 2. Pattern negation
// 3. Pattern Or(|)
// 4. Errors functional tests
//    a. Transition in --> arm
//    b. Unreachable arms error / arm precedence
//    c. Duplicate state names
//    d. Inconsistent action args
//    e. Inconsistent action error checks
//    f. Reconsume in sequence (?)
// 5. Non-streaming parser
// 6. JSON POC

// v0.3.0
// 1. Skip optimisation

// v0.4.0
// 1. GrarphViz
// 2. TracingGraphViz

// v1.0.0
// 1. Other optimisations
