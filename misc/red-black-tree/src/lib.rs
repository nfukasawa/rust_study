#![feature(box_syntax, box_patterns)]

use std::cmp::Ordering;
use std::fmt;
use std::mem;

pub struct RBTreeSet<T: Ord> {
    root: Tr<T>,
}

impl<T> RBTreeSet<T>
where
    T: Ord + fmt::Debug + fmt::Display,
{
    pub fn new() -> RBTreeSet<T> {
        RBTreeSet { root: Tr::E }
    }

    pub fn contains(&self, val: &T) -> bool {
        self.root.contains(val)
    }
    
    pub fn len(&self) -> u64 {
        self.root.len()
    }

    pub fn insert(&mut self, val: T) -> bool {
        let (root, result) = mem::replace(&mut self.root, Tr::E).insert(val);
        self.root = root;
        if result {
            self.root.black();
        }
        result
    }
}

#[derive(Debug)]
enum C {
    R,
    B,
}

#[derive(Debug)]
enum Tr<T: Ord> {
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

    fn black(&mut self) {
        match self {
            Tr::N(v, c, l, r) => {
                *c = C::B;
            }
            Tr::E => (),
        }
    }

    fn contains(&self, val: &T) -> bool {
        match self {
            Tr::N(v, c, l, r) => match v.cmp(val) {
                Ordering::Equal => true,
                Ordering::Less => r.contains(val),
                Ordering::Greater => l.contains(val),
            },
            Tr::E => false,
        }
    }

    fn len(&self) ->u64 {
        match self {
            Tr::N(v, c, l, r) => l.len() + r.len() + 1,
            Tr::E => 0,
        }
    }

    fn insert(self, val: T) -> (Tr<T>, bool) {
        match self {
            Tr::N(v, c, l, r) => match v.cmp(&val) {
                Ordering::Equal => (Tr::N(v, c, l, r), false),
                Ordering::Less => {
                    let (r1, b) = r.insert(val);
                    (Tr::N(v, c, l, box r1).balance(), b)
                }
                Ordering::Greater => {
                    let (l1, b) = l.insert(val);
                    (Tr::N(v, c, box l1, r), b)
                }
            },
            Tr::E => (Tr::new(val), true),
        }
    }

    #[rustfmt::skip]
    fn balance(self) -> Tr<T> {
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
}
