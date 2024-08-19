#[cfg(test)]
mod stack_tests {
    use rusty_data_structures::ds::Stack;

    #[test]
    fn creational_test() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(stack.top(), None);
    }

    #[test]
    fn insertion_test() {
        let mut stack: Stack<i32> = Stack::new();
        stack.push(4);
        assert_eq!(stack.top(), Some(4));

        stack.push(5);
        assert_eq!(stack.top(), Some(5));

        stack.push(6);
        stack.push(10);

        assert_eq!(stack.top(), Some(10));
    }

    #[test]
    fn deletion_test() {
        let mut stack: Stack<i32> = Stack::new();

        stack.push(10);
        assert_eq!(stack.top(), Some(10));

        stack.push(15);
        assert_eq!(stack.top(), Some(15));

        stack.pop();
        assert_eq!(stack.top(), Some(10));

        stack.pop();
        assert_eq!(stack.top(), None);
    }
}
