use std::{cell::RefCell, rc::Rc};

// Stack definition

type StackLink<T> = Option<Box<StackNode<T>>>;

#[derive(Debug)]
pub struct Stack<T> {
    top: StackLink<T>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StackNode<T> {
    pub value: T,
    pub pred: StackLink<T>,
}

///////////////////////////////////////////
// Queue definition

type QueueLink<T> = Option<Rc<RefCell<QueueNode<T>>>>;

#[derive(Debug)]
pub struct Queue<T> {
    front: QueueLink<T>,
    rear: QueueLink<T>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct QueueNode<T> {
    pub value: T,
    pub next: QueueLink<T>,
}

mod queue;
mod stack;
