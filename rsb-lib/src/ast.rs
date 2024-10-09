//! Abstract Syntax Tree
//! AST for the reversed polish notation of this project.
//!
//!Based on [Writing An Interpreter In Go](https://interpreterbook.com)
use std::ops::{Not, BitAnd, BitOr, BitXor};

pub type BoolNode = Node<bool>;

pub enum Node<T> {
    Leaf(T),
    TwoOp {
        left: Box<Node<T>>,
        right: Box<Node<T>>,
        op: Box<dyn Fn(T, T) -> T>,
    },
    OneOp {
        left: Box<Node<T>>,
        op: Box<dyn Fn(T) -> T>,
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
            Self::OneOp { left, op } => op(left.value()),
            Self::TwoOp { left, right, op } => op(left.value(), right.value()),
        }
    }

}

impl<T> Node<T> 
where
    T: Not<Output = T> + BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T> + PartialEq + Eq + From<bool>
{
    pub fn negation<L>(left: L) -> Self
    where
        L: Into<Node<T>>,
    {
        Self::OneOp {
            left: Box::new(left.into()),
            op: Box::new(|l| !l),
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
        }
    }
}

impl<T> From<T> for Node<T> {
    fn from(value: T) -> Self {
        Self::Leaf(value)
    }
}
