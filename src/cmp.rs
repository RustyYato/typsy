use crate::{
    call::{CallMut, CallOnce, Not},
    coprod::{CoCons, CoNil},
    hlist::{Cons, Nil, NonEmpty},
};

pub trait Any<'a, F, TagList = ()> {
    fn any(&'a self, f: F) -> bool;
}

pub trait All<'a, F, TagList = ()> {
    fn all(&'a self, f: F) -> bool;
}

impl<'a, F, T: Any<'a, Not<F>, TagList>, TagList> All<'a, F, TagList> for T {
    fn all(&'a self, f: F) -> bool { !self.any(Not(f)) }
}

impl<'a, F> Any<'a, F, ()> for Nil {
    fn any(&'a self, _: F) -> bool { false }
}

impl<'a, F> Any<'a, F, ()> for CoNil {
    fn any(&'a self, _: F) -> bool { match *self {} }
}

impl<'a, F, T: 'a> Any<'a, F> for Cons<T, Nil>
where
    F: CallOnce<(&'a T,), Output = bool>,
{
    fn any(&'a self, f: F) -> bool { f.call_once((&self.value,)) }
}

impl<'a, F, T: 'a, R: NonEmpty> Any<'a, F> for Cons<T, R>
where
    F: CallMut<(&'a T,), Output = bool>,
    R: Any<'a, F>,
{
    fn any(&'a self, mut f: F) -> bool { f.call_mut((&self.value,)) || self.rest.any(f) }
}

impl<'a, F, T: 'a, R> Any<'a, F> for CoCons<T, R>
where
    F: CallOnce<(&'a T,), Output = bool>,
    R: Any<'a, F>,
{
    fn any(&'a self, f: F) -> bool {
        match self {
            Self::Value(value) => f.call_once((value,)),
            Self::Rest(rest) => rest.any(f),
        }
    }
}

impl<'a, F, T: 'a, N> Any<'a, F, (N, ())> for Cons<T, Nil>
where
    F: CallOnce<(&'a T,), N, Output = bool>,
{
    fn any(&'a self, f: F) -> bool { f.call_once((&self.value,)) }
}

impl<'a, F, T: 'a, R: NonEmpty, N, M> Any<'a, F, (N, M)> for Cons<T, R>
where
    F: CallMut<(&'a T,), N, Output = bool>,
    R: Any<'a, F, M>,
{
    fn any(&'a self, mut f: F) -> bool { f.call_mut((&self.value,)) || self.rest.any(f) }
}

impl<'a, F, T: 'a, R, N, M> Any<'a, F, (N, M)> for CoCons<T, R>
where
    F: CallOnce<(&'a T,), N, Output = bool>,
    R: Any<'a, F, M>,
{
    fn any(&'a self, f: F) -> bool {
        match self {
            Self::Value(value) => f.call_once((value,)),
            Self::Rest(rest) => rest.any(f),
        }
    }
}
