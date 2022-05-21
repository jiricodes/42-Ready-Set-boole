use crate::boolrpntree::BoolRpnTree;

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
    let rpn_tree = BoolRpnTree::from(formula);
    rpn_tree.result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
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
            assert_eq!(*exp, eval_formula(*test), "Case: \"{}\"", *test);
        }
    }

    #[test]
    fn subject() {
        let tests: Vec<(bool, &str)> = vec![
            (false, "10&"),
            (true, "10|"),
            (true, "11>"),
            (false, "10="),
            (true, "1011||="),
        ];
        for (exp, test) in tests.iter() {
            assert_eq!(*exp, eval_formula(*test), "Case: \"{}\"", *test);
        }
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

    #[test]
    #[should_panic]
    fn invalid_stack_short_00() {
        assert_eq!(false, eval_formula("1&"));
    }

    #[test]
    #[should_panic]
    fn invalid_stack_short_01() {
        assert_eq!(false, eval_formula("!"));
    }
}
