use std::{cell::RefCell, rc::Rc};

// ***
// Doubly Linked List
// ***
type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: i32,
    next: Link,
    back: Link,
}

struct TransactionLog {
    head: Link,
    tail: Link,
    pub length: u64,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            back: None,
        }))
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
            Some(old) => {
                old.borrow_mut().next = Some(new.clone());
                new.borrow_mut().back = Some(old.clone());
            }
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // Take head if there is one, and look at next
        // make next the new head
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                // How do we deal with back?
                // if you have a next borrow it's back and set to None
                next.borrow_mut().back = None;
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

    pub fn iter(&self) -> ListIterator {
        ListIterator::new(self.head.clone())
    }

    pub fn back_iter(self) -> ListIterator {
        ListIterator::new(self.tail)
    }
}

pub struct ListIterator {
    current: Link,
}

impl ListIterator {
    fn new(start_at: Link) -> ListIterator {
        ListIterator { current: start_at }
    }
}

impl Iterator for ListIterator {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        let current = &self.current;
        let mut result = None;
        // Series of unpacking
        // Rc -- unpack -> RefCell -- unpack -> Node -- unpack -> value
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            }
            None => None,
        };
        result
    }
}

impl IntoIterator for TransactionLog {
    type Item = i32;
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.head)
    }
}

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<i32> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.back.clone()
            }
            None => None,
        };
        result
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
        let log: TransactionLog = TransactionLog::new_empty();
        assert_eq!(log.length, 0);
    }

    #[test]
    fn transaction_log_iterate_i32() {
        let mut log = TransactionLog::new_empty();

        for i in 1..10 {
            log.append(10 * i);
        }

        let mut index = 1;
        for n in log {
            assert_eq!(n, index * 10);
            index += 1;
        }
    }

    #[test]
    fn transaction_log_iterate_rev_i32() {
        let mut log = TransactionLog::new_empty();

        for i in 1..=10 {
            log.append(10 * i);
        }

        let mut index = 10;
        for n in log.back_iter() {
            assert_eq!(n, index * 10);
            index -= 1;
        }
    }

    #[test]
    fn transaction_log_with_items_i32() {
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
}
