#![no_std]

/// Very much like [fold](core::iter::Iterator::fold), only allows one to break conditionally.
/// If condition is never met, behaves exactly like [fold](core::iter::Iterator::fold)
pub trait Spool<T, F, A>
where
    T: Iterator,
    F: FnMut(A, T::Item) -> Fuse<A>,
{
    fn spool(self, init: A, f: F) -> A;
}

impl<T, F, A> Spool<T, F, A> for T
where
    T: Iterator,
    F: FnMut(A, T::Item) -> Fuse<A>,
{
    fn spool(self, init: A, mut f: F) -> A {
        let mut acc = init;

        for item in self {
            match f(acc, item) {
                Fuse::Continue(c) => acc = c,
                Fuse::Break(ret) => return ret,
            }
        }

        acc
    }
}

/// A [Fuse] is an enum that can be used to escape from a [Spool] fold.
pub enum Fuse<T> {
    Continue(T),
    Break(T),
}
