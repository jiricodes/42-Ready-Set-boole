use std::cmp::{Eq, PartialEq};
use std::ops::{BitAnd, BitOr, BitXor, Not};

pub enum ExpNode<T> {
    Leaf(T),
    TwoOp {
        left: Box<ExpNode<T>>,
        right: Box<ExpNode<T>>,
        op: Box<dyn Fn(T, T) -> T>,
    },
    OneOp {
        left: Box<ExpNode<T>>,
        op: Box<dyn Fn(T) -> T>,
    },
}

impl<T> ExpNode<T>
where
    T: Not<Output = T>,
{
    pub fn negation<L>(left: L) -> Self
    where
        L: Into<ExpNode<T>>,
    {
        Self::OneOp {
            left: Box::new(left.into()),
            op: Box::new(|l| !l),
        }
    }
}

impl<T> ExpNode<T>
where
    T: BitAnd<Output = T>,
{
    pub fn and<L, R>(left: L, right: R) -> Self
    where
        L: Into<ExpNode<T>>,
        R: Into<ExpNode<T>>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| l & r),
        }
    }
}

impl<T> ExpNode<T>
where
    T: BitOr<Output = T>,
{
    pub fn or<L, R>(left: L, right: R) -> Self
    where
        L: Into<ExpNode<T>>,
        R: Into<ExpNode<T>>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| l | r),
        }
    }
}

impl<T> ExpNode<T>
where
    T: BitXor<Output = T>,
{
    pub fn xor<L, R>(left: L, right: R) -> Self
    where
        L: Into<ExpNode<T>>,
        R: Into<ExpNode<T>>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| l ^ r),
        }
    }
}

impl<T> ExpNode<T>
where
    T: Not<Output = T> + BitAnd<Output = T>,
{
    pub fn mat_condition<L, R>(left: L, right: R) -> Self
    where
        L: Into<ExpNode<T>>,
        R: Into<ExpNode<T>>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| !(l & !r)),
        }
    }
}

impl<T> ExpNode<T>
where
    T: PartialEq + Eq + From<bool>,
{
    pub fn eq<L, R>(left: L, right: R) -> Self
    where
        L: Into<ExpNode<T>>,
        R: Into<ExpNode<T>>,
    {
        Self::TwoOp {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            op: Box::new(|l, r| (l == r).into()),
        }
    }
}
