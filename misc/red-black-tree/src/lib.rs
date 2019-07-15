#![feature(box_syntax, box_patterns)]

use std::cmp::Ordering;
use std::fmt;
use std::mem;

pub struct RBTreeSet<T> {
    root: Tr<T>,
    len: u64,
}

impl<T> RBTreeSet<T>
where
    T: Ord + fmt::Debug + fmt::Display,
{
    pub fn new() -> RBTreeSet<T> {
        RBTreeSet {
            root: Tr::E,
            len: 0,
        }
    }

    pub fn contains(&self, val: &T) -> bool {
        self.root.contains(val)
    }

    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn insert(&mut self, val: T) -> bool {
        let (root, result) = mem::replace(&mut self.root, Tr::E).insert(val);
        self.root = root;
        if result {
            self.root.black();
            self.len += 1;
        }
        result
    }
}

#[derive(Debug)]
enum C {
    R,
    B,
}


impl C
{
    fn is_black(&self) -> bool {
        match self {
            C::B => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
enum Tr<T> {
    N(T, C, Box<Tr<T>>, Box<Tr<T>>),
    E,
}

impl<T> Tr<T>
where
    T: Ord + fmt::Debug + fmt::Display,
{
    fn new(val: T) -> Tr<T> {
        Tr::N(val, C::R, box Tr::E, box Tr::E)
    }

    fn is_nil(&self) -> bool {
        match self {
            Tr::E => true,
            _ => false,
        }
    }

    fn black(&mut self) {
        match self {
            Tr::N(_, c, _, _) => {
                *c = C::B;
            }
            Tr::E => (),
        }
    }

    fn contains(&self, val: &T) -> bool {
        match self {
            Tr::N(v, _, l, r) => match v.cmp(val) {
                Ordering::Equal => true,
                Ordering::Less => r.contains(val),
                Ordering::Greater => l.contains(val),
            },
            Tr::E => false,
        }
    }

    fn insert(self, val: T) -> (Tr<T>, bool) {
        match self {
            Tr::N(v, c, l, r) => match v.cmp(&val) {
                Ordering::Equal => (Tr::N(v, c, l, r), false),
                Ordering::Less => {
                    let (r1, b) = r.insert(val);
                    (Tr::N(v, c, l, box r1).balance_insert(), b)
                }
                Ordering::Greater => {
                    let (l1, b) = l.insert(val);
                    (Tr::N(v, c, box l1, r).balance_insert(), b)
                }
            },
            Tr::E => (Tr::new(val), true),
        }
    }

    #[rustfmt::skip]
    fn balance_insert(self) -> Tr<T> {
        match self {
            // LL
            Tr::N(v0, C::B, box Tr::N(v1, C::R, box Tr::N(v2, C::R, l2, r2), r1), r0)
                => Tr::N(v1, C::R, box Tr::N(v2, C::B, l2, r2), box Tr::N(v0, C::B, r1, r0)),
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

    fn remove(self, val: &T) -> (Tr<T>, bool) {
        match self {
            Tr::N(v, c, l, r) => match v.cmp(&val) {
                Ordering::Equal => match (*l, *r) {
                    (Tr::E, Tr::E) => (Tr::E, false),
                    (l, Tr::E) => (l, true),
                    (Tr::E, r) => (r, true),
                    (l, r) => {
                        let (l1, v1, c1) = l.remove_max();
                        (Tr::N(v1, c1, box l1, box r).balance_remove_left(), true)
                    }
                },
                Ordering::Less => {
                    let (n, b) = r.remove(val);
                    (n.balance_remove_right(), b)
                },
                Ordering::Greater => {
                    let (n,b) = l.remove(val);
                    (n.balance_remove_left(), b)
                }
            },
            Tr::E => (self, false),
        }
    }

    fn balance_remove_left(self) -> Tr<T> {
        // TODO
        return self
    }

    fn balance_remove_right(self) -> Tr<T> {
        // TODO
        return self
    }

    #[rustfmt::skip]
    fn remove_max(self) -> (Tr<T>, T, C) {
        match self {
            Tr::N(v,c,l,box Tr::N(v1, c1, l1, box Tr::E))
                => (Tr::N(v,c,l, box Tr::E), v1, c1),
            Tr::N(v,c,l,box Tr::E)
                => (*l, v, c),
            Tr::N(v,c,l,r) => {
                let (node, v1, c1) = r.remove_max();
                (Tr::N(v, c, l, box node), v1, c1)
            }
            Tr::E => panic!("node must not be empty")
        }
    }
}
