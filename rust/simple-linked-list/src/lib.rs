use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut node = &self.head;

        let mut n = 0;
        loop {
            if node.is_none() {
                break;
            }
            n += 1;
            let t = node.as_ref().unwrap();
            node = &t.next;
        }
        n
    }

    pub fn push(&mut self, element: T) {
        let new_node = Node {
            next: self.head.take(),
            data: element,
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            return Some(node.data);
        }
        None
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T>
    where
        T: Copy,
    {
        let mut linked_list = SimpleLinkedList::new();
        let mut node = &self.head;
        while node.is_some() {
            let t = node.as_ref().unwrap();
            linked_list.push(t.data);
            node = &t.next;
        }
        linked_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter()
            .fold(SimpleLinkedList::new(), |mut accumulator, element| {
                accumulator.push(element);
                return accumulator;
            })
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut result = Vec::new();
        while let Some(element) = linked_list.pop() {
            result.insert(0, element);
        }
        result
    }
}
