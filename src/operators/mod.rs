pub mod arithmetic;
pub mod control_flow;
pub mod relational_logical;

use crate::mutant::Mutator;

/// Returns the default set of mutation operators.
pub fn default_operators() -> Vec<Box<dyn Mutator>> {
    vec![
        Box::new(arithmetic::ArithmeticOperatorMutator),
        Box::new(control_flow::ControlFlowMutator),
        Box::new(relational_logical::RelationalOperatorMutator),
        Box::new(relational_logical::LogicalOperatorMutator),
        Box::new(relational_logical::ConditionNegationMutator),
    ]
}
