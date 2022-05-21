mod adder;
use adder::adder;

mod multiplier;
use multiplier::multiplier;

mod gray_code;
use gray_code::gray_code;

mod boolean_evaluation;
use boolean_evaluation::eval_formula;

pub mod boolrpntree;
mod exptree;

mod truth_table;
pub use truth_table::print_truth_table;

mod sat;
