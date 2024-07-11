#[derive(Debug)]
pub struct Stack<T> {
    pub top: Option<Box<Node<T>>>,
    pub heigh: usize,
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    pub pred: Option<Box<Node<T>>>,
}

pub mod stack;
