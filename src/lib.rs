#![no_std]

use core::{
    fmt::{self, Debug},
    ops::{Deref, DerefMut},
};

mod iter;

use iter::{Iter, IterMut};

pub trait PointerFamily {
    type Pointer<'a, T: ?Sized + 'a>: DerefMut<Target = T>;
}

#[derive(Debug)]
struct RefFamily;

impl PointerFamily for RefFamily {
    type Pointer<'a, T: ?Sized + 'a> = &'a mut T;
}

#[cfg(feature = "alloc")]
mod with_std {
    extern crate alloc;
    use alloc::boxed::Box;

    use crate::PointerFamily;

    pub struct BoxFamily;

    impl PointerFamily for BoxFamily {
        type Pointer<'a, T: ?Sized + 'a> = Box<T>;
    }
}

#[cfg(feature = "alloc")]
pub use with_std::BoxFamily;

pub struct Node<'a, T: ?Sized + 'a, P: 'a>
where
    P: PointerFamily,
{
    next: Option<P::Pointer<'a, Node<'a, T, P>>>,
    elem: T,
}

impl<'a, T: 'a, P: 'a> Node<'a, T, P>
where
    P: PointerFamily,
{
    pub fn new(elem: T) -> Self {
        Self { next: None, elem }
    }

    pub fn into_inner(self) -> T {
        self.elem
    }
}

impl<'a, T: 'a, P: 'a> Deref for Node<'a, T, P>
where
    P: PointerFamily,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.elem
    }
}

impl<'a, T: 'a, P: 'a> DerefMut for Node<'a, T, P>
where
    P: PointerFamily,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elem
    }
}

impl<'a, T: 'a, P: 'a> Debug for Node<'a, T, P>
where
    T: Debug,
    P: PointerFamily,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node").field("elem", &self.elem).finish()
    }
}

pub struct List<'a, T: ?Sized + 'a, P: 'a>
where
    P: PointerFamily,
{
    head: Option<P::Pointer<'a, Node<'a, T, P>>>,
}

impl<'a, T: 'a, P: 'a> List<'a, T, P>
where
    P: PointerFamily,
{
    pub const fn new() -> Self {
        Self { head: None }
    }

    pub const fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push(&mut self, mut node: P::Pointer<'a, Node<'a, T, P>>) {
        node.next = self.head.take();
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<P::Pointer<'a, Node<'a, T, P>>> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node
        })
    }

    pub fn iter<'b>(&'b self) -> Iter<'b, 'a, T, P>
    where
        'a: 'b,
    {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut<'b>(&'b mut self) -> IterMut<'b, 'a, T, P>
    where
        'a: 'b,
    {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

// impl <'a, T: ?Sized + 'a, P: 'a> Drop for List<'a, T, P>
// where
//     P: PointerFamily
// {
//     fn drop(&mut self) {
//         let mut cur_link = self.head.take();

//         while let Some(mut node) = cur_link {
//             cur_link = node.next.take();
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::{List, Node, RefFamily};

    #[test]
    fn test_mut_refs() {
        let mut l: List<i32, RefFamily> = List::new();
        let mut n: Node<i32, RefFamily> = Node::new(20);
        let mut n2: Node<i32, RefFamily> = Node::new(30);

        l.push(&mut n);
        l.push(&mut n2);

        assert_eq!(**l.pop().unwrap(), 30);
        assert_eq!(**l.pop().unwrap(), 20);
        assert_eq!(l.is_empty(), true);
    }
}

#[cfg(feature = "alloc")]
#[cfg(test)]
mod test_alloc {
    extern crate alloc;
    use super::{BoxFamily, List, Node};
    use alloc::boxed::Box;

    #[test]
    fn test_boxed() {
        let mut l: List<i32, BoxFamily> = List::new();
        let n: Node<i32, BoxFamily> = Node::new(20);
        let n2: Node<i32, BoxFamily> = Node::new(30);

        l.push(Box::new(n));
        l.push(Box::new(n2));

        assert_eq!(**l.pop().unwrap(), 30);
        assert_eq!(**l.pop().unwrap(), 20);
        assert_eq!(l.is_empty(), true);
    }
}
