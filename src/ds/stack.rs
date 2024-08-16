use super::{Stack, StackLink, StackNode};

impl<T: Copy + Clone + PartialEq> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { top: None }
    }

    pub fn is_empty(&self) -> bool {
        self.top == None
    }

    pub fn top(&self) -> Option<T> {
        match &self.top {
            Option::Some(node) => Some(node.value),
            Option::None => None,
        }
    }

    pub fn push(&mut self, value: T) {
        match &self.top {
            Option::Some(node) => {
                self.top = new_link(value, Some(node.clone()));
            }
            Option::None => {
                self.top = new_link(value, None);
            }
        };
    }

    pub fn pop(&mut self) {
        match &self.top {
            Some(node) => {
                self.top = node.pred.clone();
            }
            None => (),
        }
    }
}

impl<T: Copy> StackNode<T> {
    fn new(value: T, pred: Option<Box<StackNode<T>>>) -> StackNode<T> {
        StackNode { value, pred }
    }
}

fn new_link<T: Copy>(value: T, predecessor: Option<Box<StackNode<T>>>) -> StackLink<T> {
    Some(Box::new(StackNode::new(value, predecessor)))
}
