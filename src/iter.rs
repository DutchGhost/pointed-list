use crate::{Node, PointerFamily};

pub struct Iter<'a, 'b, T: 'a, P>
where
    P: PointerFamily + 'a,
{
    pub(crate) next: Option<&'a Node<'b, T, P>>,
}

impl<'a, 'b, T, P> Iterator for Iter<'a, 'b, T, P>
where
    P: PointerFamily,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, 'b, T: 'a, P>
where
    P: PointerFamily + 'a,
{
    pub(crate) next: Option<&'a mut Node<'b, T, P>>,
}

impl<'a, 'b, T, P> Iterator for IterMut<'a, 'b, T, P>
where
    P: PointerFamily,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}
