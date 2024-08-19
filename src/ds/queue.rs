use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    ops::Deref,
    rc::Rc,
};

use super::{Queue, QueueLink, QueueNode};

impl<T: Display + Debug> Display for Queue<T> {
    // add code here
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut temp = self.front.clone();

        write!(f, "[")?;
        loop {
            match temp {
                Some(ref node) => {
                    let ref_cell_node = node.clone();
                    let next_node = &ref_cell_node.borrow().next;
                    let node_value = &ref_cell_node.borrow().value;

                    match next_node {
                        None => write!(f, "{:?}", node_value)?,
                        Some(_) => write!(f, "{:?}, ", node_value)?,
                    }

                    temp = next_node.clone();
                }
                None => {
                    write!(f, "]")?;
                    break;
                }
            }
        }

        Ok(())
    }
}

impl<T: Copy + Clone + PartialEq + std::fmt::Debug> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            front: None,
            rear: None,
        }
    }

    pub fn enqueue(&mut self, enqueued_value: T) {
        let enqueued_node = new_link(enqueued_value);

        match &self.rear {
            None => {
                self.rear = enqueued_node.clone();
                self.front = enqueued_node;
            }
            Some(old_rear_node) => {
                old_rear_node.deref().borrow_mut().next = enqueued_node.clone();
                self.rear = enqueued_node;
            }
        }
    }

    pub fn dequeue(&mut self) {
        match &self.front {
            Some(node) => {
                let first_node = node.clone();
                self.front = first_node.deref().borrow_mut().next.clone();
            }
            None => (),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.front != Option::None
    }
}

impl<T: Copy> QueueNode<T> {
    fn new(value: T, next: QueueLink<T>) -> QueueNode<T> {
        QueueNode { value, next }
    }
}

fn new_link<T: Copy>(value: T) -> QueueLink<T> {
    Some(Rc::new(RefCell::new(QueueNode::new(value, None))))
}
