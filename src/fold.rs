use coprod::Uninhabitted;

use crate::{
    call::{AlwaysOk, CallMut, CallOnce},
    coprod,
    hlist::{Cons, HList, Nil, NonEmpty},
    CoProd,
};

pub trait TryFold<A, F, TagList> {
    type Output;
    type Error;

    fn try_fold(self, acc: A, f: F) -> Result<Self::Output, Self::Error>;
}

pub trait Fold<A, F, TagList> {
    type Output;

    fn fold(self, acc: A, f: F) -> Self::Output;
}

impl<A, F, T, TagList> Fold<A, F, TagList> for T
where
    T: TryFold<A, AlwaysOk<F>, TagList>,
    T::Error: Uninhabitted,
{
    type Output = <T as TryFold<A, AlwaysOk<F>, TagList>>::Output;

    fn fold(self, acc: A, f: F) -> Self::Output {
        match self.try_fold(acc, AlwaysOk(f)) {
            Ok(value) => value,
            Err(err) => err.uninhabitted(),
        }
    }
}

impl<A, F> TryFold<A, F, ()> for Nil {
    type Output = A;
    type Error = core::convert::Infallible;

    fn try_fold(self, acc: A, _: F) -> Result<Self::Output, Self::Error> { Ok(acc) }
}

impl<A, F, T, O, E, N> TryFold<A, F, N> for Cons<T, Nil>
where
    F: CallOnce<(A, T), N, Output = Result<O, E>>,
{
    type Output = O;
    type Error = CoProd!(E);

    fn try_fold(self, acc: A, f: F) -> Result<Self::Output, Self::Error> {
        let acc = f.call_once((acc, self.value)).map_err(coprod::CoCons::Value)?;
        Ok(acc)
    }
}

impl<A, F, T, R: HList, O, E, N, M> TryFold<A, F, (N, M)> for Cons<T, R>
where
    F: CallMut<(A, T), N, Output = Result<O, E>>,
    R: NonEmpty + TryFold<O, F, M>,
{
    type Output = R::Output;
    type Error = CoProd!(E, @R::Error);

    fn try_fold(self, acc: A, mut f: F) -> Result<Self::Output, Self::Error> {
        let acc = f.call_mut((acc, self.value)).map_err(coprod::CoCons::Value)?;
        let acc = self.rest.try_fold(acc, f).map_err(coprod::CoCons::Rest)?;
        Ok(acc)
    }
}
