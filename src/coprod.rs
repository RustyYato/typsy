use crate::peano;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoNil {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoCons<T, R> {
    Value(T),
    Rest(R),
}

#[macro_export]
macro_rules! CoProd {
    () => { $crate::coprod::CoNil };
    (, @$last:ty) => { $last };
    ($first:ty $(, $rest:ty)* $(, @$last:ty)* $(,)?) => { $crate::coprod::CoCons<$first, $crate::CoProd!($($rest),* $(, @$last)?)> };
}

pub trait CoProd: Sized + crate::Seal {}
impl crate::Seal for CoNil {}
impl CoProd for CoNil {}

impl<T, R: crate::Seal> crate::Seal for CoCons<T, R> {}
impl<T, R: CoProd> CoProd for CoCons<T, R> {}

pub trait Uninhabitted {
    fn uninhabitted(self) -> !;
}

impl Uninhabitted for core::convert::Infallible {
    fn uninhabitted(self) -> ! { match self {} }
}

impl Uninhabitted for CoNil {
    fn uninhabitted(self) -> ! { match self {} }
}

impl<T: Uninhabitted, R: Uninhabitted> Uninhabitted for CoCons<T, R> {
    fn uninhabitted(self) -> ! {
        match self {
            CoCons::Value(value) => value.uninhabitted(),
            CoCons::Rest(rest) => rest.uninhabitted(),
        }
    }
}

pub trait Access<T, N>: CoProd {
    type Remainder: CoProd;

    fn take(self) -> Result<T, Self::Remainder>;

    fn put(value: T) -> Self;
}

pub trait IntoSuperset<T: CoProd, N>: CoProd {
    fn into_superset(self) -> T;
}

pub trait IntoSubset<T: CoProd, N>: CoProd {
    type Remainder: CoProd;

    fn into_subset(self) -> Result<T, Self::Remainder>;
}

impl<T, R: CoProd> Access<T, peano::Zero> for CoCons<T, R> {
    type Remainder = R;

    fn take(self) -> Result<T, Self::Remainder> {
        match self {
            Self::Value(value) => Ok(value),
            Self::Rest(rest) => Err(rest),
        }
    }

    fn put(value: T) -> Self { Self::Value(value) }
}

impl<T, U, R: Access<T, N>, N> Access<T, peano::Succ<N>> for CoCons<U, R> {
    type Remainder = CoCons<U, R::Remainder>;

    fn take(self) -> Result<T, Self::Remainder> {
        match self {
            Self::Value(value) => Err(CoCons::Value(value)),
            Self::Rest(rest) => rest.take().map_err(CoCons::Rest),
        }
    }

    fn put(value: T) -> Self { CoCons::Rest(R::put(value)) }
}

impl<T: CoProd> IntoSuperset<T, crate::TyList!()> for CoNil {
    fn into_superset(self) -> T { match self {} }
}

impl<T, R, C, N, M> IntoSuperset<C, crate::TyList!(N, @M)> for CoCons<T, R>
where
    C: Access<T, N>,
    R: IntoSuperset<C, M>,
{
    fn into_superset(self) -> C {
        match self {
            Self::Value(value) => Access::put(value),
            Self::Rest(rest) => rest.into_superset(),
        }
    }
}

impl<T: CoProd> IntoSubset<CoNil, crate::TyList!()> for T {
    type Remainder = Self;

    fn into_subset(self) -> Result<CoNil, Self::Remainder> { Err(self) }
}

impl<T, R: CoProd, C, N, M> IntoSubset<CoCons<T, R>, crate::TyList!(N, @M)> for C
where
    C: Access<T, N>,
    C::Remainder: IntoSubset<R, M>,
{
    type Remainder = <C::Remainder as IntoSubset<R, M>>::Remainder;

    fn into_subset(self) -> Result<CoCons<T, R>, Self::Remainder> {
        match self.take() {
            Ok(value) => Ok(CoCons::Value(value)),
            Err(rest) => rest.into_subset().map(CoCons::Rest),
        }
    }
}

impl<T: CoProd, U: CoProd, N> Shuffle<U, N> for T where T: IntoSubset<U, N, Remainder = CoNil> {}
pub trait Shuffle<T: CoProd, N>: IntoSubset<T, N, Remainder = CoNil> {
    fn shuffle(self) -> T {
        match self.into_subset() {
            Ok(x) => x,
            Err(x) => {
                let _: CoNil = x;
                match x {}
            }
        }
    }
}

pub trait AnonResult<Err, N> {
    type Ok;

    fn lift_err(self) -> Result<Self::Ok, Err>;
}

impl<T, E, F: CoProd, N> AnonResult<F, N> for Result<T, E>
where
    E: IntoSuperset<F, N>,
{
    type Ok = T;

    fn lift_err(self) -> Result<T, F> { self.map_err(IntoSuperset::into_superset) }
}
