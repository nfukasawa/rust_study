pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut c = 0;
        let mut cur = &self.head;
        while let Some(node) = cur {
            c += 1;
            cur = &node.next;
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
        match &self.head {
            Some(node) => Some(&node.data),
            None => None,
        }
    }

    pub fn push_back(&mut self, _elm: T) {
        let mut cur = &mut self.head;
        while let Some(node) = cur {
            cur = &mut node.next;
        }
        cur.replace(Box::new(Node::new(_elm, None)));
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_opt().map(|node| node.data)
    }

    fn pop_back_opt(&mut self) -> Option<Box<Node<T>>> {
        if let Some(head) = &self.head {
            if head.is_tail() {
                return self.head.take();
            }
        }

        let mut cur = &mut self.head;
        while let Some(node) = cur {
            if node.next_is_tail() {
                return node.next.take();
            }
            cur = &mut node.next;
        }
        None
    }

    pub fn peek_back(&self) -> Option<&T> {
        let mut cur = &self.head;
        while let Some(node) = cur {
            if node.is_tail() {
                return Some(&node.data);
            }
            cur = &node.next;
        }
        None
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut cur = &self.head;
        while let Some(node) = cur {
            list.push_front(node.data.clone());
            cur = &node.next;
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
        match self.next {
            None => true,
            _ => false,
        }
    }

    fn next_is_tail(&self) -> bool {
        match &self.next {
            None => false,
            Some(node) => node.is_tail(),
        }
    }
}
