pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, _elm: T) {
        self.push_front(_elm)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.peek_front()
    }

    pub fn push_front(&mut self, _elm: T) {
        self.len += 1;
        self.head = Some(Box::new(Node::new(_elm, self.head.take())));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.len -= 1;
            self.head = node.next;
            node.data
        })
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn push_back(&mut self, _elm: T) {
        unimplemented!()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unimplemented!()
    }

    pub fn peek_back(&mut self, _elm: T) {
        unimplemented!()
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut cur = self.head.as_ref().map(|node| node);
        while let Some(node) = cur {
            list.push(node.data.clone());
            cur = node.next.as_ref().map(|node| node);
        }
        list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_elms: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();
        for elm in _elms {
            list.push(elm.clone());
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = vec![];
        let mut cur = self.head;
        while let Some(node) = cur {
            vec.push(node.data);
            cur = node.next;
        }
        vec.reverse();
        vec
    }
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(elm: T, next: Option<Box<Node<T>>>) -> Self {
        Node {
            data: elm,
            next: next,
        }
    }
}
