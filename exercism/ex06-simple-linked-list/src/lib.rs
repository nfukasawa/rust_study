pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        match self.head.as_ref() {
            Some(node) => node.count(1),
            None => 0,
        }
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
        self.head = Some(Box::new(Node::new(_elm, self.head.take())));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn push_back(&mut self, _elm: T) {
        let tail = Some(Box::new(Node::new(_elm, None)));
        match self.head.as_mut() {
            Some(node) => node.push_back(tail),
            None => self.head = tail,
        };
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.head.as_mut() {
            Some(node) => match node.next {
                None => self.head.take(),
                _ => node.pop_back(),
            }
            .map(|node| node.data),
            None => None,
        }
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.head.as_ref().map(|node| node.peek_back())
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut cur = self.head.as_ref();
        while let Some(node) = cur {
            list.push_front(node.data.clone());
            cur = node.next.as_ref();
        }
        list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_elms: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();
        for elm in _elms {
            list.push_front(elm.clone());
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

    fn count(&self, c: usize) -> usize {
        match self.next.as_ref() {
            Some(node) => node.count(c + 1),
            None => c,
        }
    }

    fn push_back(&mut self, tail: Option<Box<Node<T>>>) {
        match self.next.as_mut() {
            Some(node) => node.push_back(tail),
            None => self.next = tail,
        }
    }

    fn pop_back(&mut self) -> Option<Box<Node<T>>> {
        match self.next.as_mut() {
            Some(node) => match node.next {
                None => self.next.take(),
                _ => node.pop_back(),
            },
            None => None,
        }
    }

    fn peek_back(&self) -> &T {
        match self.next.as_ref() {
            Some(node) => node.peek_back(),
            None => &self.data,
        }
    }
}
