//! Abstract Syntax Tree
//! AST for the reversed polish notation of this project.
//!
//!Based on [Writing An Interpreter In Go](https://interpreterbook.com)
use std::fmt::Debug;
use std::iter::Enumerate;
use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    False,    // 0 ⊥ false
    True,     // 1 ⊤ true
    Negation, // ! ¬ Negation
    And,      // & ∧ Conjunction
    Or,       // | ∨ Disjunction
    Xor,      // ˆ ⊕ Exclusive disjunction
    Cond,     // > ⇒ Material condition
    Eq,       // = ⇔ Logical equivalence
    EOF,
    Illegal,
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '0' => Self::False,
            '1' => Self::True,
            '!' => Self::Negation,
            '&' => Self::And,
            '|' => Self::Or,
            '^' => Self::Xor,
            '>' => Self::Cond,
            '=' => Self::Eq,
            _ => Self::Illegal,
        }
    }
}

pub struct Lexer<'a> {
    iter: Enumerate<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            iter: input.chars().enumerate(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        if let Some((pos, char)) = self.iter.next() {
            Token::from(char)
        } else {
            Token::EOF
        }
    }
}

pub enum Node {
    Leaf(bool),
    TwoOp {
        left: Box<Node>,
        right: Box<Node>,
        op: Box<dyn Fn(bool, bool) -> bool>,
    },
    OneOp {
        left: Box<Node>,
        op: Box<dyn Fn(bool) -> bool>,
    },
}

impl Node {
    pub fn new(value: bool) -> Self {
        Self::Leaf(value)
    }

    fn new_boxed(value: bool) -> Box<Self> {
        Box::new(Self::Leaf(value))
    }

    pub fn value(self) -> bool {
        match self {
            Self::Leaf(value) => value,
            Self::OneOp { left, op } => op(left.value()),
            Self::TwoOp { left, right, op } => op(left.value(), right.value()),
        }
    }

    fn negation<L>(left: L) -> Self
    where
        L: Into<Node>,
    {
        Self::OneOp {
            left: Box::new(left.into()),
            op: Box::new(|l| !l),
        }
    }

    fn and<L, R>(left: L, right: R) -> Self
    where
        L: Into<Node>,
        R: Into<Node>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| l & r),
        }
    }

    fn or<L, R>(left: L, right: R) -> Self
    where
        L: Into<Node>,
        R: Into<Node>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| l | r),
        }
    }

    fn xor<L, R>(left: L, right: R) -> Self
    where
        L: Into<Node>,
        R: Into<Node>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| l ^ r),
        }
    }

    fn cond<L, R>(left: L, right: R) -> Self
    where
        L: Into<Node>,
        R: Into<Node>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| !l | r),
        }
    }

    fn eq<L, R>(left: L, right: R) -> Self
    where
        L: Into<Node>,
        R: Into<Node>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| l == r),
        }
    }
}

impl From<bool> for Node {
    fn from(value: bool) -> Self {
        Self::Leaf(value)
    }
}

impl<'a> From<Lexer<'a>> for Node {
    fn from(mut lexer: Lexer) -> Self {
        let mut rpn_stack: Vec<Self> = Vec::new();
        let mut current_token = lexer.next_token();
        while current_token != Token::EOF {
            match current_token {
                Token::False => rpn_stack.push(false.into()),
                Token::True => rpn_stack.push(true.into()),
                Token::Negation => {
                    let node = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Negation");
                    rpn_stack.push(Self::negation(node));
                }
                Token::And => {
                    let right = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Conjunction");
                    let left = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Conjunction");
                    rpn_stack.push(Self::and(left, right));
                }
                Token::Or => {
                    let right = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Disjunction");
                    let left = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Disjunction");
                    rpn_stack.push(Self::or(left, right));
                }
                Token::Xor => {
                    let right = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Exclusive Disjunction");
                    let left = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Exclusive Disjunction");
                    rpn_stack.push(Self::xor(left, right));
                }
                Token::Cond => {
                    let right = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Material Condition");
                    let left = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Material Condition");
                    rpn_stack.push(Self::cond(left, right));
                }
                Token::Eq => {
                    let right = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Logical Equivalence");
                    let left = rpn_stack
                        .pop()
                        .expect("Invalid stack: Missing operand for Logical Equivalence");
                    rpn_stack.push(Self::eq(left, right));
                }
                Token::EOF | Token::Illegal => {
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
    fn test_next_token() {
        let input = "01!&|^>=7";
        let mut lexer = Lexer::new(&input);

        assert_eq!(lexer.next_token(), Token::False);
        assert_eq!(lexer.next_token(), Token::True);
        assert_eq!(lexer.next_token(), Token::Negation);
        assert_eq!(lexer.next_token(), Token::And);
        assert_eq!(lexer.next_token(), Token::Or);
        assert_eq!(lexer.next_token(), Token::Xor);
        assert_eq!(lexer.next_token(), Token::Cond);
        assert_eq!(lexer.next_token(), Token::Eq);
        assert_eq!(lexer.next_token(), Token::Illegal);
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    #[test]
    #[should_panic]
    fn test_node_from_lexer_panic() {
        let input = "01!&|^>=7";
        let mut lexer = Lexer::new(&input);
        let rpn: Node = lexer.into();
        let _ = rpn.value();
    }

    #[test]
    fn test_rpn_basic() {
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
            let rpn: Node = lexer.into();
            assert_eq!(*exp, rpn.value(), "Case: \"{}\"", *test);
        }
    }

    #[test]
    fn test_rpn_subject() {
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
            let rpn: Node = lexer.into();
            assert_eq!(*exp, rpn.value(), "Case: \"{}\"", *test);
        }
    }
}
