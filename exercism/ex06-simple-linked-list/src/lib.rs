pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut cur = self.head.as_ref();
        let mut c = 0;
        while let Some(node) = cur {
            c += 1;
            cur = node.next.as_ref();
        }
        c
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
        let mut cur = self.head.as_mut();
        while let Some(node) = cur {
            if node.is_tail() {
                node.next = tail;
                return;
            }
            cur = node.next.as_mut();
        }
        self.head = tail;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_opt().map(|node| node.data)
    }

    fn pop_back_opt(&mut self) -> Option<Box<Node<T>>> {
        if let Some(head) = self.head.as_ref() {
            if head.is_tail() {
                return self.head.take();
            }
        }

        let mut cur = self.head.as_mut();
        while let Some(node) = cur {
            if node.next_is_tail() {
                return node.next.take();
            }
            cur = node.next.as_mut();
        }
        None
    }

    pub fn peek_back(&self) -> Option<&T> {
        let mut cur = self.head.as_ref();
        while let Some(node) = cur {
            if node.is_tail() {
                return Some(&node.data);
            }
            cur = node.next.as_ref();
        }
        None
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

    fn is_tail(&self) -> bool {
        match self.next.as_ref() {
            None => true,
            _ => false,
        }
    }

    fn next_is_tail(&self) -> bool {
        match self.next.as_ref() {
            None => false,
            Some(node) => node.is_tail(),
        }
    }
}
