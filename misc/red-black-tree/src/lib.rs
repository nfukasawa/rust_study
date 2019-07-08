#![feature(box_syntax, box_patterns)]

use std::cmp::Ordering;
use std::mem;

pub struct RBTree<T: Ord> {
    root: Tr<T>,
}

impl<T> RBTree<T>
where
    T: Ord,
{
    pub fn new() -> RBTree<T> {
        RBTree { root: Tr::E }
    }

    pub fn contains(&self, val: &T) -> bool {
        self.contains(val)
    }

    pub fn insert(&mut self, val: T) -> bool {
        if self.root.contains(&val) {
            false
        } else {
            let node = mem::replace(&mut self.root, Tr::E);
            self.root = node.insert(val);
            true
        }
    }
}

enum C {
    R,
    B,
}

enum Tr<T: Ord> {
    N(T, C, Box<Tr<T>>, Box<Tr<T>>),
    E,
}

impl<T> Tr<T>
where
    T: Ord,
{
    fn new(val: T) -> Tr<T> {
        Tr::N(val, C::R, box Tr::E, box Tr::E)
    }

    fn new_root(val: T) -> Tr<T> {
        Tr::N(val, C::B, box Tr::E, box Tr::E)
    }

    fn contains(&self, val: &T) -> bool {
        match self {
            Tr::N(v, c, l, r) => match v.cmp(val) {
                Ordering::Equal => true,
                Ordering::Less => l.contains(val),
                Ordering::Greater => r.contains(val),
            },
            _ => false,
        }
    }

    fn insert(self, val: T) -> Tr<T> {
        match self {
            Tr::N(v, c, l, r) => match v.cmp(&val) {
                Ordering::Equal => Tr::N(v, c, l, r),
                Ordering::Less => Tr::N(v, c, box l.insert(val), r).balance(),
                Ordering::Greater => Tr::N(v, c, l, box r.insert(val)).balance(),
            },
            _ => Tr::new(val),
        }
    }

    #[rustfmt::skip]
    fn balance(self) -> Tr<T> {
        match self {
            // LL
            Tr::N(v0, C::B, box Tr::N(v1, C::R, box Tr::N(v2, C::R, l2, r2), r1), r0)
                => Tr::N(v1, C::R, box Tr::N(v0, C::B, l2, r2), box Tr::N(v2, C::B, r1, r0)),
            // LR
            Tr::N(v0, C::B, box Tr::N(v1, C::R, l1, box Tr::N(v2, C::R, l2, r2)), r0)
                => Tr::N(v2, C::R, box Tr::N(v1, C::B, l1, l2), box Tr::N(v0, C::B, r2, r0)),
            // RL
            Tr::N(v0, C::B, l0, box Tr::N(v1, C::R, box Tr::N(v2, C::R, l2, r2), r1)) 
                => Tr::N(v2, C::R, box Tr::N(v0, C::B, l0, l2), box Tr::N(v1, C::B, r2, r1)),
            // RR
            Tr::N(v0, C::B, l0, box Tr::N(v1, C::R, l1, box Tr::N(v2, C::R, l2, r2))) 
                => Tr::N(v1, C::R, box Tr::N(v0, C::B, l0, l1), box Tr::N(v2, C::B, l2, r2)),
            _ => self,
        }
    }
}

#[test]
fn test() {
    let mut tree = RBTree::new();
    assert!(tree.insert(1));
    assert!(tree.insert(2));
    assert!(tree.insert(3));
    assert!(tree.insert(4));
    assert!(!tree.insert(1));
}
