use crate::BoolNode;
use crate::Lexer;
use crate::token::BoolToken;
use crate::lexer::LexerTrait;



impl<'a> From<Lexer<'a>> for BoolNode {
    fn from(mut lexer: Lexer) -> Self {
        let mut rpn_stack: Vec<Self> = Vec::new();
        let mut current_token: BoolToken = lexer.next_token();
        while current_token != BoolToken::EOF {
            match current_token {
                BoolToken::False => rpn_stack.push(false.into()),
                BoolToken::True => rpn_stack.push(true.into()),
                BoolToken::Negation => {
                    let node = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Negation");
                    rpn_stack.push(Self::negation(node));
                }
                BoolToken::And => {
                    let right = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Conjunction");
                    let left = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Conjunction");
                    rpn_stack.push(Self::and(left, right));
                }
                BoolToken::Or => {
                    let right = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Disjunction");
                    let left = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Disjunction");
                    rpn_stack.push(Self::or(left, right));
                }
                BoolToken::Xor => {
                    let right = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Exclusive Disjunction");
                    let left = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Exclusive Disjunction");
                    rpn_stack.push(Self::xor(left, right));
                }
                BoolToken::Cond => {
                    let right = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Material Condition");
                    let left = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Material Condition");
                    rpn_stack.push(Self::cond(left, right));
                }
                BoolToken::Eq => {
                    let right = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Logical Equivalence");
                    let left = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Logical Equivalence");
                    rpn_stack.push(Self::eq(left, right));
                }
                BoolToken::EOF | BoolToken::Illegal => {
                    panic!("Unexpected token");
                }
            }
            current_token = lexer.next_token();
        }
        if rpn_stack.len() != 1 {
            panic!("Invalid formula: Missing operations. Stack lenght should be 1 when reached EOF, but is {}.", rpn_stack.len());
        }
        rpn_stack.pop().expect("Invalid formula: Stack is empty")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_node_from_lexer_panic() {
        let input = "01!&|^>=7";
        let lexer = Lexer::new(&input);
        let rpn: BoolNode = lexer.into();
        let _ = rpn.value();
    }

    #[test]
    fn test_rpn_bool_basic() {
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
            let lexer = Lexer::new(test);
            let rpn: BoolNode = lexer.into();
            assert_eq!(*exp, rpn.value(), "Case: \"{}\"", *test);
        }
    }

    #[test]
    fn test_rpn_bool_subject() {
        let tests: Vec<(bool, &str)> = vec![
            (true, "10|1&"),
            (true, "101|&"),
            (false, "10&"),
            (true, "10|"),
            (true, "11>"),
            (false, "10="),
            (true, "1011||="),
        ];
        for (exp, test) in tests.iter() {
            let lexer = Lexer::new(test);
            let rpn: BoolNode = lexer.into();
            assert_eq!(*exp, rpn.value(), "Case: \"{}\"", *test);
        }
    }
}
