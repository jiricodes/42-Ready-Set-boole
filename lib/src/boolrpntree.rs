use crate::exptree::ExpNode;

pub struct BoolRpnTree {
    tree: ExpNode<bool>,
}

impl From<&str> for BoolRpnTree {
    fn from(formula: &str) -> Self {
        let mut stack: Vec<ExpNode<bool>> = Vec::new();
        for c in formula.chars() {
            match c {
                '0' => stack.push(ExpNode::new(false)),
                '1' => stack.push(ExpNode::new(true)),
                '!' => Self::stack_add_negation(&mut stack),
                '&' => Self::stack_add_conjunction(&mut stack),
                '|' => Self::stack_add_disjunction(&mut stack),
                '^' => Self::stack_add_exclusive_disjunction(&mut stack),
                '>' => Self::stack_add_material_condition(&mut stack),
                '=' => Self::stack_add_logical_equivalence(&mut stack),
                _ => panic!("Invalid formula: Unexpected character \'{}\'", c),
            }
        }
        if stack.len() != 1 {
            panic!(
                "Invalid formula: Missing operation (stack len = {})",
                stack.len()
            );
        }
        Self {
            tree: stack.pop().unwrap(),
        }
    }
}

fn stack_len_check(len: usize, exp: usize, name: &str) {
    if len < exp {
        panic!(
            "Invalid formula: Called {} on invalid stack (len {:?} expected {})",
            name, len, exp
        );
    }
}

impl BoolRpnTree {
    fn stack_add_negation(stack: &mut Vec<ExpNode<bool>>) {
        stack_len_check(stack.len(), 1, "Negation");
        let node = stack.pop().unwrap();
        stack.push(ExpNode::neg(node));
    }

    fn stack_add_conjunction(stack: &mut Vec<ExpNode<bool>>) {
        stack_len_check(stack.len(), 1, "Conjunction");
        let right = stack.pop().unwrap();
        let left = stack.pop().unwrap();
        stack.push(ExpNode::and(left, right));
    }

    fn stack_add_disjunction(stack: &mut Vec<ExpNode<bool>>) {
        stack_len_check(stack.len(), 2, "Disjunction");
        let right = stack.pop().unwrap();
        let left = stack.pop().unwrap();
        stack.push(ExpNode::or(left, right));
    }

    fn stack_add_exclusive_disjunction(stack: &mut Vec<ExpNode<bool>>) {
        stack_len_check(stack.len(), 2, "Exclusive disjunction");
        let right = stack.pop().unwrap();
        let left = stack.pop().unwrap();
        stack.push(ExpNode::xor(left, right));
    }

    fn stack_add_material_condition(stack: &mut Vec<ExpNode<bool>>) {
        stack_len_check(stack.len(), 2, "Material condition");
        let right = stack.pop().unwrap();
        let left = stack.pop().unwrap();
        stack.push(ExpNode::mat_condition(left, right));
    }

    fn stack_add_logical_equivalence(stack: &mut Vec<ExpNode<bool>>) {
        stack_len_check(stack.len(), 2, "Logical equivalence");
        let right = stack.pop().unwrap();
        let left = stack.pop().unwrap();
        stack.push(ExpNode::eq(left, right));
    }

    /// Consumes the struct and gives result
    pub fn result(self) -> bool {
        self.tree.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_basic() {
        let tests: Vec<(bool, &str)> = vec![
            (false, "0"),
            (true, "1"),
            (false, "1!"),
            (false, "10&"),
            (true, "10|"),
            (true, "10^"),
            (false, "00^"),
            (false, "10>"),
            (true, "01>"),
            (true, "11="),
        ];
        for (exp, test) in tests.iter() {
            let rpn = BoolRpnTree::from(*test);
            assert_eq!(*exp, rpn.result(), "Case: \"{}\"", *test);
        }
    }
}
