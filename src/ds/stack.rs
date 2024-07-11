use super::{Node, Stack};

impl<T: Copy + Clone> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            top: None,
            heigh: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.heigh == 0
    }

    pub fn top(&mut self) -> Option<T> {
        match &self.top {
            Option::Some(node) => Some(node.get()),
            Option::None => None,
        }
    }

    pub fn push(&mut self, value: T) {
        self.heigh += 1;
        match &self.top {
            Option::Some(node) => {
                self.top = Some(Box::new(Node::new(value, Some(node.clone()))));
            }
            Option::None => {
                self.top = Some(Box::new(Node::new(value, None)));
            }
        };
    }

    pub fn pop(&mut self) {
        if self.heigh == 0 {
            return;
        }
        self.heigh -= 1;
        match &self.top {
            Some(node) => {
                self.top = node.pred.clone();
            }
            None => (),
        }
    }
}

impl<T: Copy> Node<T> {
    pub fn new(value: T, pred: Option<Box<Node<T>>>) -> Node<T> {
        Node { value, pred }
    }

    pub fn get(&self) -> T {
        self.value
    }
}
