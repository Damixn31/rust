use std::fmt::Debug;

#[derive(Debug)]
pub struct BinaryTree<T> {
    pub root: Option<Box<Node<T>>>,
}
#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}
