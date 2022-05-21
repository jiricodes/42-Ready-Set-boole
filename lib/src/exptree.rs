use std::ops::{Not, BitAnd, BitOr, BitXor};
use std::cmp::{PartialEq, Eq};
pub struct BinData<T> {
    data: T,
    left: BinTree<T>,
    right: BinTree<T>,
}
pub struct BinTree<T>(Option<Box<BinData<T>>>);

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }
}

pub enum ExpNode<T> {
    Leaf(T),
    Branch {
        left: Box<ExpNode<T>>,
        right: Box<ExpNode<T>>,
        op: Box<dyn Fn(T, T) -> T>,
    },
}

impl<T> ExpNode<T>
where T: Not, BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T> + PartialEq;
{

}