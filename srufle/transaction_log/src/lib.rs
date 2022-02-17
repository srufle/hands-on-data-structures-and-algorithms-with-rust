use std::{cell::RefCell, rc::Rc};

// Why are we using and Rc<RefCell> and not a Box
// Box is for single ownership
// Rc<RefCell> is for multiple ownership with interior mutability
// https://tekshinobi.com/rust-tips-box-rc-arc-cell-refcell-mutex
type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: i32,
    next: SingleLink,
}

struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

impl TransactionLog {
    pub fn new_empty() -> Self {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: i32) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }
}
#[cfg(test)]
mod tests {
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    use super::*;
    #[test]
    fn create_node() {
        let test_value = 10;
        let node = Node::new(test_value);
        let node = node.borrow();
        assert_eq!(node.value, test_value);
    }

    #[test]
    fn empty_transaction_log() {
        let log = TransactionLog::new_empty();
        assert_eq!(log.length, 0);
    }

    #[test]
    fn transaction_log_with_items() {
        let mut log = TransactionLog::new_empty();
        assert_eq!(log.length, 0);
        log.append(10);
        assert_eq!(log.length, 1);
        log.append(20);
        assert_eq!(log.length, 2);
    }
}