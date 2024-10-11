//! Abstract Syntax Tree
//! AST for the reversed polish notation of this project.
//!
//!Based on [Writing An Interpreter In Go](https://interpreterbook.com)
use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, BitXor, Not},
};

/// This is used to solve boolean evaluations
pub type BoolNode = Node<bool>;

pub enum Node<T> {
    Leaf(T),
    TwoOp {
        left: Box<Node<T>>,
        right: Box<Node<T>>,
        op: Box<dyn Fn(T, T) -> T>,
        name: &'static str,
    },
    OneOp {
        left: Box<Node<T>>,
        op: Box<dyn Fn(T) -> T>,
        name: &'static str,
    },
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self::Leaf(value)
    }

    fn new_boxed(value: T) -> Box<Self> {
        Box::new(Self::Leaf(value))
    }

    pub fn value(self) -> T {
        match self {
            Self::Leaf(value) => value,
            Self::OneOp { left, op, name: _ } => op(left.value()),
            Self::TwoOp {
                left,
                right,
                op,
                name: _,
            } => op(left.value(), right.value()),
        }
    }
}

impl<T> Node<T>
where
    T: Not<Output = T>
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + BitXor<Output = T>
        + PartialEq
        + Eq
        + From<bool>,
{
    pub fn negation<L>(left: L) -> Self
    where
        L: Into<Node<T>>,
    {
        Self::OneOp {
            left: Box::new(left.into()),
            op: Box::new(|l| !l),
            name: "NEG",
        }
    }

    pub fn and<L, R>(left: L, right: R) -> Self
    where
        L: Into<Node<T>>,
        R: Into<Node<T>>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| l & r),
            name: "AND",
        }
    }

    pub fn or<L, R>(left: L, right: R) -> Self
    where
        L: Into<Node<T>>,
        R: Into<Node<T>>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| l | r),
            name: "OR",
        }
    }

    pub fn xor<L, R>(left: L, right: R) -> Self
    where
        L: Into<Node<T>>,
        R: Into<Node<T>>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| l ^ r),
            name: "XOR",
        }
    }

    pub fn cond<L, R>(left: L, right: R) -> Self
    where
        L: Into<Node<T>>,
        R: Into<Node<T>>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| !l | r),
            name: "COND",
        }
    }

    pub fn eq<L, R>(left: L, right: R) -> Self
    where
        L: Into<Node<T>>,
        R: Into<Node<T>>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| (l == r).into()),
            name: "EQ",
        }
    }
}

impl<T> From<T> for Node<T> {
    fn from(value: T) -> Self {
        Self::Leaf(value)
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Leaf(val) => write!(f, "{}", val),
            Node::OneOp { left, op: _, name } => write!(f, "{}({})", name, left),
            Node::TwoOp {
                left,
                right,
                op: _,
                name,
            } => write!(f, "{}({}, {})", name, left, right),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Op {
    Neg,
    And,
    Or,
    Xor,
    Cond,
    Eq,
}

impl Op {
    fn as_str(&self) -> &'static str {
        match self {
            Op::Neg => "Neg",
            Op::And => "And",
            Op::Or => "Or",
            Op::Xor => "Xor",
            Op::Cond => "Cond",
            Op::Eq => "Eq",
        }
    }

    fn as_symbol(&self) -> &'static str {
        match self {
            Op::Neg => "!",
            Op::And => "&",
            Op::Or => "|",
            Op::Xor => "^",
            Op::Cond => ">",
            Op::Eq => "=",
        }
    }
}

/// This is used for generic conversions without evaluations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VarNode {
    Leaf(char),
    TwoOp {
        left: Box<VarNode>,
        right: Box<VarNode>,
        op: Op,
    },
    OneOp {
        left: Box<VarNode>,
        op: Op,
    },
}

impl VarNode {
    pub fn new(value: char) -> VarNode {
        VarNode::Leaf(value)
    }

    pub fn negation<L>(left: L) -> Self
    where
        L: Into<VarNode>,
    {
        Self::OneOp {
            left: Box::new(left.into()),
            op: Op::Neg,
        }
    }

    pub fn and<L, R>(left: L, right: R) -> Self
    where
        L: Into<VarNode>,
        R: Into<VarNode>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Op::And,
        }
    }

    pub fn or<L, R>(left: L, right: R) -> Self
    where
        L: Into<VarNode>,
        R: Into<VarNode>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Op::Or,
        }
    }

    pub fn xor<L, R>(left: L, right: R) -> Self
    where
        L: Into<VarNode>,
        R: Into<VarNode>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Op::Xor,
        }
    }

    pub fn cond<L, R>(left: L, right: R) -> Self
    where
        L: Into<VarNode>,
        R: Into<VarNode>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Op::Cond,
        }
    }

    pub fn eq<L, R>(left: L, right: R) -> Self
    where
        L: Into<VarNode>,
        R: Into<VarNode>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Op::Eq,
        }
    }

    fn is_op(&self, op_in: Op) -> bool {
        match self {
            Self::Leaf(_) => false,
            Self::OneOp { left: _, op }
            | Self::TwoOp {
                left: _,
                right: _,
                op,
            } => *op == op_in,
        }
    }

    fn is_leaf(&self) -> bool {
        match self {
            Self::Leaf(_) => true,
            _ => false,
        }
    }

    fn get_left(&self) -> Option<VarNode> {
        match self {
            Self::Leaf(_) => None,
            Self::OneOp { left, op: _ }
            | Self::TwoOp {
                left,
                right: _,
                op: _,
            } => Some(*left.clone()),
        }
    }

    fn get_right(&self) -> Option<VarNode> {
        match self {
            Self::Leaf(_) | Self::OneOp { left: _, op: _ } => None,
            Self::TwoOp {
                left: _,
                right,
                op: _,
            } => Some(*right.clone()),
        }
    }

    fn reduce_children(&mut self) {
        match *self {
            VarNode::Leaf(_) => {}
            VarNode::OneOp {
                ref mut left,
                op: _,
            } => {
                let new_left = VarNode::reduce(*left.clone());
                *left = Box::new(new_left);
            }
            VarNode::TwoOp {
                ref mut left,
                ref mut right,
                op: _,
            } => {
                let new_left = VarNode::reduce(*left.clone());
                *left = Box::new(new_left);
                let new_right = VarNode::reduce(*right.clone());
                *right = Box::new(new_right);
            }
        }
    }

    fn has_nnf_forbidden(&self) -> bool {
        match self {
            Self::Leaf(_) => return false,
            Self::OneOp { left, op } => {
                if *op == Op::Neg && !left.is_leaf() {
                    true
                } else {
                    left.has_nnf_forbidden()
                }
            }
            Self::TwoOp { left, right, op } => {
                if (*op == Op::Cond) | (*op == Op::Eq) {
                    true
                } else {
                    left.has_nnf_forbidden() | right.has_nnf_forbidden()
                }
            }
        }
    }

    pub fn nnf_reduce(&mut self) {
        let mut not_finished = self.has_nnf_forbidden();
        while not_finished {
            *self = Self::reduce(self.clone());
            not_finished = self.has_nnf_forbidden();
        }
    }

    pub fn reduce(node: VarNode) -> VarNode {
        let mut new_node = node.clone();
        match node {
            VarNode::Leaf(_) => {}
            VarNode::OneOp { left, op } => {
                if op == Op::Neg {
                    if left.is_op(Op::Neg) {
                        // Double Neg
                        new_node = left.get_left().expect("Get left on OneOp node failed");
                    } else if left.is_op(Op::And) {
                        // De Morgans AND
                        let new_child_left = VarNode::negation(
                            left.get_left().expect("Get left on TwoOp node failed"),
                        );
                        let new_child_right = VarNode::negation(
                            left.get_right().expect("Get right on TwoOp node failed"),
                        );
                        new_node = VarNode::or(new_child_left, new_child_right);
                    } else if left.is_op(Op::Or) {
                        // De Morgans OR
                        let new_child_left = VarNode::negation(
                            left.get_left().expect("Get left on TwoOp node failed"),
                        );
                        let new_child_right = VarNode::negation(
                            left.get_right().expect("Get right on TwoOp node failed"),
                        );
                        new_node = VarNode::and(new_child_left, new_child_right);
                    }
                }
            }
            VarNode::TwoOp { left, right, op } => {
                if op == Op::Cond {
                    // Material conditions
                    new_node = VarNode::or(VarNode::negation(*left.clone()), *right.clone());
                } else if op == Op::Eq {
                    // Equivalence
                    new_node = VarNode::and(
                        VarNode::cond(*left.clone(), *right.clone()),
                        VarNode::cond(*right.clone(), *left.clone()),
                    );
                } else if op == Op::And {
                    // AND Distributivity
                    if left.is_op(Op::Or) {
                        let a = left.get_left().expect("Get left on TwoOp node failed");
                        let b = left.get_right().expect("Get right on TwoOp node failed");
                        // c == right
                        new_node = VarNode::or(
                            VarNode::and(a, *right.clone()),
                            VarNode::and(b, *right.clone()),
                        );
                    } else if right.is_op(Op::Or) {
                        let b = right.get_left().expect("Get left on TwoOp node failed");
                        let c = right.get_right().expect("Get right on TwoOp node failed");
                        // a == left
                        new_node = VarNode::or(
                            VarNode::and(*left.clone(), b),
                            VarNode::and(*left.clone(), c),
                        );
                    }
                } else if op == Op::Or {
                    // OR Distributivity
                    if left.is_op(Op::And) {
                        let a = left.get_left().expect("Get left on TwoOp node failed");
                        let b = left.get_right().expect("Get right on TwoOp node failed");
                        // c == right
                        new_node = VarNode::and(
                            VarNode::or(a, *right.clone()),
                            VarNode::or(b, *right.clone()),
                        );
                    } else if right.is_op(Op::And) {
                        let b = right.get_left().expect("Get left on TwoOp node failed");
                        let c = right.get_right().expect("Get right on TwoOp node failed");
                        // a == left
                        new_node = VarNode::and(
                            VarNode::or(*left.clone(), b),
                            VarNode::or(*left.clone(), c),
                        );
                    }
                }
            }
        }
        new_node.reduce_children();
        new_node
    }
}

impl From<char> for VarNode {
    fn from(value: char) -> Self {
        Self::Leaf(value)
    }
}

impl Display for VarNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VarNode::Leaf(val) => write!(f, "{}", val),
            VarNode::OneOp { left, op } => write!(f, "{}{}", left, op.as_symbol()),
            VarNode::TwoOp { left, right, op } => write!(f, "{}{}{}", left, right, op.as_symbol()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Lexer;

    use super::*;

    #[test]
    fn test_nnf() {
        let formula = "A!!";
        let expected = "A";
        let mut rpn: VarNode = Lexer::new(formula).into();
        rpn.nnf_reduce();
        assert_eq!(expected, format!("{}", rpn), "Original {}", formula);

        let formula = "AB&!";
        let expected = "A!B!|";
        let mut rpn: VarNode = Lexer::new(formula).into();
        rpn.nnf_reduce();
        assert_eq!(expected, format!("{}", rpn), "Original {}", formula);

        let formula = "AB|!";
        let expected = "A!B!&";
        let mut rpn: VarNode = Lexer::new(formula).into();
        rpn.nnf_reduce();
        assert_eq!(expected, format!("{}", rpn), "Original {}", formula);

        let formula = "AB>";
        let expected = "A!B|";
        let mut rpn: VarNode = Lexer::new(formula).into();
        rpn.nnf_reduce();
        assert_eq!(expected, format!("{}", rpn), "Original {}", formula);

        let formula = "AB=";
        let expected = "A!B|B!A|&";
        let mut rpn: VarNode = Lexer::new(formula).into();
        rpn.nnf_reduce();
        assert_eq!(expected, format!("{}", rpn), "Original {}", formula);

        let formula = "AB|C&!";
        let expected = "A!B!&C!|";
        let mut rpn: VarNode = Lexer::new(formula).into();
        rpn.nnf_reduce();
        assert_eq!(expected, format!("{}", rpn), "Original {}", formula);

        let formula = "AB>!";
        let expected = "AB!&";
        let mut rpn: VarNode = Lexer::new(formula).into();
        rpn.nnf_reduce();
        assert_eq!(expected, format!("{}", rpn), "Original {}", formula);
    }
}
