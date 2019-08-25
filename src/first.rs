use std::mem;

pub struct List {
    head: Link,
}

#[derive(Debug, PartialEq)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug, PartialEq)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty), // Fortune and glory, kid. Fortune and glory.
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[macro_export]
macro_rules! list {
    ( $($elem:expr),+ ) => {{
        let mut list = List::new();
        $(
            list.push($elem);
        )+
        list
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_push_element() {
        let mut list = List::new();
        list.push(42);
        assert_eq!(
            Link::More(Box::new(Node {
                elem: 42,
                next: Link::Empty,
            })),
            list.head
        );
    }

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(None, list.pop());

        let mut list = list![1, 2, 3];
        assert_eq!(Some(3), list.pop());
        assert_eq!(Some(2), list.pop());

        list.push(4);
        list.push(5);
        assert_eq!(Some(5), list.pop());
        assert_eq!(Some(4), list.pop());
        assert_eq!(Some(1), list.pop());
        assert_eq!(None, list.pop());
    }
}
