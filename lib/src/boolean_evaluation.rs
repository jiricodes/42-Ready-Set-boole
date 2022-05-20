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
    let mut stack = BoolRpnStack::new();
    for char in formula.chars() {
        stack.eval(char);
    }
    stack.result()
}

struct BoolRpnStack {
    stack: Vec<bool>,
}

impl BoolRpnStack {
    fn new() -> Self {
        Self {
            stack: Vec::with_capacity(2),
        }
    }

    fn negation(&mut self) {
        self.op_len_check(1, "Negation");
        self.stack[0] = !self.stack[0];
    }

    fn conjunction(&mut self) {
        self.op_len_check(2, "Conjunction");
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(a & b);
    }

    fn disjunction(&mut self) {
        self.op_len_check(2, "Disjunction");
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(a | b);
    }

    fn exclusive_disjunction(&mut self) {
        self.op_len_check(2, "Exclusive disjunction");
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(a ^ b);
    }

    fn material_condition(&mut self) {
        self.op_len_check(2, "Material condition");
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(if a && !b { false } else { true });
    }

    fn logical_equivalence(&mut self) {
        self.op_len_check(2, "Logical equivalence");
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(a == b);
    }

    fn op_len_check(&self, exp: usize, op: &str) {
        if self.stack.len() < exp {
            panic!(
                "Invalid formula: Called {} on invalid stack (len {:?} expected {})",
                op,
                self.stack.len(),
                exp
            );
        }
    }

    fn eval(&mut self, c: char) {
        match c {
            '0' => self.stack.push(false),
            '1' => self.stack.push(true),
            '!' => self.negation(),
            '&' => self.conjunction(),
            '|' => self.disjunction(),
            '^' => self.exclusive_disjunction(),
            '>' => self.material_condition(),
            '=' => self.logical_equivalence(),
            _ => panic!("Invalid formula: Unexpected character \'{}\'", c),
        }
    }

    fn result(&self) -> bool {
        if self.stack.len() != 1 {
            panic!(
                "Invalid formula: Unexpected result call (stack.len() = {})",
                self.stack.len()
            );
        }
        self.stack[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!(false, eval_formula("0"));
        assert_eq!(true, eval_formula("1"));
        assert_eq!(false, eval_formula("1!"));
        assert_eq!(false, eval_formula("10&"));
        assert_eq!(true, eval_formula("10|"));
        assert_eq!(true, eval_formula("10^"));
        assert_eq!(false, eval_formula("00^"));
        assert_eq!(false, eval_formula("10>"));
        assert_eq!(true, eval_formula("01>"));
        assert_eq!(true, eval_formula("11="));
    }

    #[test]
    fn subject() {
        assert_eq!(false, eval_formula("10&"));
        assert_eq!(true, eval_formula("10|"));
        assert_eq!(true, eval_formula("11>"));
        assert_eq!(false, eval_formula("10="));
        assert_eq!(true, eval_formula("1011||="));
    }

    #[test]
    #[should_panic]
    fn invalid_char_00() {
        eval_formula("a");
    }

    #[test]
    #[should_panic]
    fn invalid_char_01() {
        eval_formula("10!x");
    }

    #[test]
    #[should_panic]
    fn invalid_ambiguous_result() {
        assert_eq!(false, eval_formula("10"));
    }
}
