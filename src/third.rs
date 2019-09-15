use std::rc::Rc;

// A persistent stack
pub struct List<T> {
    head: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
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

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|node| & **node),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| & **node);
            &node.elem
        })
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

    #[test]
    fn iter() {
        let list = List::new()
            .append(1)
            .append(2)
            .append(3);
        let mut iter = list.iter();
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.next());
    }
}
