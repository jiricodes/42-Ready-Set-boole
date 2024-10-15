mod adder;
mod ast;
mod graycode;
mod multiplier;
mod powerset;
mod rpn_bool;
mod token;
mod lexer;

pub use adder::adder;
pub use graycode::gray_code;
pub use multiplier::multiplier;
pub use powerset::powerset;
pub use token::Token;
pub use lexer::Lexer;
pub use ast::{BoolNode, VarNode};


pub fn eval_formula(formula: &str) -> bool {
    let rpn: BoolNode = Lexer::new(formula).into();
    rpn.value()
}

pub fn negation_normal_form(formula: &str) -> String {
    let mut rpn: VarNode = Lexer::new(formula).into();
    rpn.nnf_reduce();
    format!("{}", rpn)
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    let mut rpn: VarNode = Lexer::new(formula).into();
    rpn.cnf_reduce();
    dbg!(&rpn);
    format!("{}", rpn)
}

fn print_header(labels: &Vec<char>) {
    let mut line = String::new();
    let mut underscore = String::new();
    for c in labels.iter() {
        line += &format!("| {} ", c);
        underscore += "|---";
    }
    println!("{}| = |", line);
    println!("{}|---|", underscore);
}

pub fn print_truth_table(formula: &str) {
    let formula: String = formula.to_uppercase();
    // extract all letters
    let mut vars: Vec<char> = formula
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();
    vars.sort();
    vars.dedup();
    let l = vars.len();
    print_header(&vars);
    for i in 0..2u32.pow(l as u32) {
        let mut current = formula.clone();
        let mut table_line = String::new();
        for (n, c) in vars.iter().enumerate() {
            let val = if i & (1 << (l - 1 - n)) != 0 {
                table_line += "| 1 ";
                "1"
            } else {
                table_line += "| 0 ";
                "0"
            };
            current = current.replace(*c, val);
        }
        let ret = if eval_formula(current.as_str()) {
            "1"
        } else {
            "0"
        };
        println!("{}| {} |", table_line, ret);
    }
}

pub fn sat(formula: &str) -> bool {
    let formula: String = formula.to_uppercase();
    let mut vars: Vec<char> = formula
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();
    vars.sort();
    vars.dedup();
    let l = vars.len();
    for i in 0..2u32.pow(l as u32) {
        let mut current = formula.clone();
        for (n, c) in vars.iter().enumerate() {
            let val = if i & (1 << (l - 1 - n)) != 0 {
                "1"
            } else {
                "0"
            };
            current = current.replace(*c, val);
        }
        if eval_formula(current.as_str()) {
            return true;
        } 
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sat_subject() {
        let tests: Vec<(bool, &str)> = vec![
            (true, "AB|"),
            (true, "AB&"),
            (false, "AA!&"),
            (false, "AA^"),
        ];
        for (exp, test) in tests.iter() {
            assert_eq!(*exp, sat(*test), "Case \"{}\"", *test);
        }
    }
}
