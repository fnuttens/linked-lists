/// An Ok stack
pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(None, list.pop());

        list.push(1);
        list.push(2);
        list.push(3);
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
