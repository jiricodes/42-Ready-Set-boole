///
///
/// Input can contain only:
/// | Symbol | Description           |
/// |--------|-----------------------|
/// | 0      | false                 |
/// | 1      | true                  |
/// | !      | Negation              |
/// | &      | Conjunction           |
/// | \|     | Disjunction           |
/// | ˆ      | Exclusive disjunction |
/// | >      | Material condition    |
/// | =      | Logical equivalence   |
pub fn eval_formula(formula: &str) -> bool {
    let mut stack: Vec<bool> = Vec::with_capacity(2);
    for char in formula.chars() {
        match char {
            '0' => stack.push(false),
            '1' => stack.push(true),
            _ => panic!("Uknown character {:?}", char),
        }
    }
    if stack.len() != 1 {
        panic!("Invalid formula: expected operand");
    }
    stack.pop().unwrap()
}

struct BoolRpnStack {
    stack: Vec<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!(false, eval_formula("0"));
    }

    #[test]
    #[should_panic]
    fn invalid_char() {
        assert_eq!(false, eval_formula("x"));
    }

    #[test]
    #[should_panic]
    fn invalid_missing_op() {
        assert_eq!(false, eval_formula("10"));
    }
}
