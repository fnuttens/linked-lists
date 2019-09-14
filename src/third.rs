use std::rc::Rc;

// A persistent stack
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn append(&self, elem: T) -> Self {
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> Self {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(None, list.head());

        let list = list.append(1).append(2).append(3);
        assert_eq!(Some(&3), list.head());

        let list = list.tail();
        assert_eq!(Some(&2), list.head());

        let list = list.tail();
        assert_eq!(Some(&1), list.head());

        let list = list.tail();
        assert_eq!(None, list.head());

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(None, list.head());
    }
}
