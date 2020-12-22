use crate::hlist::{Cons, HList, Nil};

include!(concat!(env!("OUT_DIR"), "/convert_tuple.rs"));

pub trait Convert {
    type HList: HList;

    fn into_hlist(self) -> Self::HList;
    fn from_hlist(hlist: Self::HList) -> Self;
}

impl Convert for Nil {
    type HList = Self;

    fn into_hlist(self) -> Self::HList { Self }
    fn from_hlist(Nil: Self::HList) -> Self { Self }
}

impl<T, R: HList> Convert for Cons<T, R> {
    type HList = Self;

    fn into_hlist(self) -> Self::HList { self }
    fn from_hlist(this: Self::HList) -> Self { this }
}
