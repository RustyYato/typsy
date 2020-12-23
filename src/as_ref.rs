use crate::{
    coprod::{CoCons, CoNil},
    hlist::{Cons, Nil},
};

pub trait AsRef<'a> {
    type Ref: 'a;
    type RefMut: 'a;

    fn as_ref(&'a self) -> Self::Ref;
    fn as_mut(&'a mut self) -> Self::RefMut;
}

impl AsRef<'_> for Nil {
    type Ref = Self;
    type RefMut = Self;

    fn as_ref(&self) -> Self::Ref { Self }

    fn as_mut(&mut self) -> Self::RefMut { Self }
}

impl AsRef<'_> for CoNil {
    type Ref = Self;
    type RefMut = Self;

    fn as_ref(&self) -> Self::Ref { *self }

    fn as_mut(&mut self) -> Self::RefMut { *self }
}

impl<'a, T: 'a, R: AsRef<'a>> AsRef<'a> for Cons<T, R> {
    type Ref = Cons<&'a T, R::Ref>;
    type RefMut = Cons<&'a mut T, R::RefMut>;

    fn as_ref(&'a self) -> Self::Ref {
        Cons {
            value: &self.value,
            rest: self.rest.as_ref(),
        }
    }

    fn as_mut(&'a mut self) -> Self::RefMut {
        Cons {
            value: &mut self.value,
            rest: self.rest.as_mut(),
        }
    }
}

impl<'a, T: 'a, R: AsRef<'a>> AsRef<'a> for CoCons<T, R> {
    type Ref = CoCons<&'a T, R::Ref>;
    type RefMut = CoCons<&'a mut T, R::RefMut>;

    fn as_ref(&'a self) -> Self::Ref {
        match self {
            CoCons::Value(value) => CoCons::Value(value),
            CoCons::Rest(rest) => CoCons::Rest(rest.as_ref()),
        }
    }

    fn as_mut(&'a mut self) -> Self::RefMut {
        match self {
            CoCons::Value(value) => CoCons::Value(value),
            CoCons::Rest(rest) => CoCons::Rest(rest.as_mut()),
        }
    }
}
