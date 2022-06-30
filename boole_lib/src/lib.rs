mod adder;
pub use adder::adder;
pub use adder::adder2;
pub use adder::adder_ref;

mod multiplier;
pub use multiplier::{multiplier, multiplier2, multiplier_easy, multiplier_ref};

mod gray_code;
use gray_code::gray_code;

mod boolean_evaluation;
use boolean_evaluation::eval_formula;

pub mod boolrpntree;
mod exptree;

mod truth_table;
pub use truth_table::print_truth_table;

mod sat;
