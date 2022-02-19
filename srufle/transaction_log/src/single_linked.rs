use std::{cell::RefCell, mem, rc::Rc};

// Why are we using and Rc<RefCell> and not a Box
// Box is for single ownership
// Rc<RefCell> is for multiple ownership with interior mutability
// https://tekshinobi.com/rust-tips-box-rc-arc-cell-refcell-mutex
type SingleLink<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone)]
struct Node<T> {
    value: T,
    next: SingleLink<T>,
}

struct TransactionLog<T> {
    head: SingleLink<T>,
    tail: SingleLink<T>,
    pub length: u64,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}
// When making generic I have to not implement Drop
// error[E0509]: cannot move out of type `Node<T>`, which implements the `Drop` trait
// TODO: This causes one of the test cases to error, because it stack overflows during default drop
// impl<T> Drop for Node<T> {
//     fn drop(&mut self) {
//         // replace next with empty, following the chain
//         let mut cur_next = mem::replace(&mut self.next, None);
//         while let Some(node) = cur_next {
//             cur_next = mem::replace(&mut node.borrow_mut().next, None);
//         }
//     }
// }

impl<T> TransactionLog<T> {
    pub fn new_empty() -> Self {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: T) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<T> {
        // Take head if the is one, and look at next
        // make next the new head
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is wrong")
                .into_inner()
                .value
        })
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    use super::*;

    //
    #[test]
    fn create_node_v1() {
        let test_value = 10;
        let node = Node::new(test_value);
        let node = node.borrow();
        assert_eq!(node.value, test_value);
    }

    #[test]
    fn empty_transaction_log_v1() {
        let log: TransactionLog<i32> = TransactionLog::new_empty();
        assert_eq!(log.length, 0);
    }

    #[test]
    fn transaction_log_v1_with_items_i32() {
        let mut log = TransactionLog::new_empty();
        assert_eq!(log.length, 0);
        log.append(10);
        assert_eq!(log.length, 1);
        log.append(20);
        assert_eq!(log.length, 2);
        log.append(30);
        assert_eq!(log.length, 3);

        let head = log.pop();
        assert_eq!(head.unwrap(), 10);
        let head = log.pop();
        assert_eq!(head.unwrap(), 20);
        let head = log.pop();
        assert_eq!(head.unwrap(), 30);
        let head = log.pop();
        assert_eq!(head, None);
    }

    #[test]
    fn transaction_log_v1_with_items_String() {
        let mut log = TransactionLog::new_empty();
        assert_eq!(log.length, 0);
        log.append(String::from("10"));
        assert_eq!(log.length, 1);
        log.append(String::from("20"));
        assert_eq!(log.length, 2);
        log.append(String::from("30"));
        assert_eq!(log.length, 3);

        let head = log.pop();
        assert_eq!(head.unwrap(), String::from("10"));
        let head = log.pop();
        assert_eq!(head.unwrap(), String::from("20"));
        let head = log.pop();
        assert_eq!(head.unwrap(), String::from("30"));
        let head = log.pop();
        assert_eq!(head, None);
    }

    #[test]
    #[ignore]
    fn transaction_v1_log_many_items() {
        let mut log = TransactionLog::new_empty();
        for n in 1..10000 {
            log.append(n * 2);
        }
    }
}
