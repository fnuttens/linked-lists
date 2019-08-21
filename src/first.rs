use std::mem;

pub struct List {
    head: Link,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
#[derive(PartialEq)]
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
}
