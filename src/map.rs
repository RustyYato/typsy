use hlist::NonEmpty;

use crate::{
    call::{Baked, CallMut, CallOnce},
    coprod, hlist, CoProd, HList,
};

pub type Mapped<T, F> = <T as Map<F>>::Output;
pub trait Map<F> {
    type Output;

    fn map(self, f: F) -> Self::Output;
}

impl<F> Map<F> for hlist::Nil {
    type Output = Self;

    fn map(self, _: F) -> Self::Output { Self }
}

impl<F> Map<F> for coprod::CoNil {
    type Output = Self;

    fn map(self, _: F) -> Self::Output { match self {} }
}

impl<F: CallOnce<(T,), N>, T, N> Map<Baked<F, N>> for hlist::Cons<T, hlist::Nil> {
    type Output = HList!(F::Output);

    fn map(self, f: Baked<F, N>) -> Self::Output { hlist!(f.call_once((self.value,))) }
}

impl<F: CallMut<(T,), N>, T, R: NonEmpty + Map<Baked<F, M>>, N, M> Map<Baked<F, (N, M)>> for hlist::Cons<T, R> {
    type Output = HList!(F::Output, @R::Output);

    fn map(self, mut f: Baked<F, (N, M)>) -> Self::Output {
        hlist!(f.call_mut((self.value,)), @self.rest.map(f.rebake()))
    }
}

impl<F: CallOnce<(T,), N>, T, R: Map<Baked<F, M>>, N, M> Map<Baked<F, (N, M)>> for coprod::CoCons<T, R> {
    type Output = CoProd!(F::Output, @R::Output);

    fn map(self, f: Baked<F, (N, M)>) -> Self::Output {
        match self {
            coprod::CoCons::Value(value) => coprod::CoCons::Value(f.call_once((value,))),
            coprod::CoCons::Rest(rest) => coprod::CoCons::Rest(rest.map(f.rebake())),
        }
    }
}
