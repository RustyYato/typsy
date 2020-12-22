use crate::peano;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nil;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cons<T, R> {
    pub value: T,
    pub rest: R,
}

pub trait HList: crate::Seal {}
impl crate::Seal for Nil {}
impl HList for Nil {}

impl<T, R: crate::Seal> crate::Seal for Cons<T, R> {}
impl<T, R: HList> HList for Cons<T, R> {}

pub trait NonEmpty: HList {}
impl<T, R: HList> NonEmpty for Cons<T, R> {}

#[macro_export]
macro_rules! HList {
    () => { $crate::hlist::Nil };
    (, @$last:ty) => { $last };
    ($first:ty $(, $rest:ty)* $(, @$last:ty)? $(,)?) => { $crate::hlist::Cons<$first, $crate::HList!($($rest),* $(, @$last)?)> };
}

#[macro_export]
macro_rules! TyList {
    ($($($item:ty),+ $(, @$last:ty)? $(,)?)?) => { $crate::HList!($($($crate::core::marker::PhantomData<$item>),+ $(, @$last)?)?) };
}

#[macro_export]
macro_rules! hlist {
    () => { $crate::hlist::Nil };
    (, @$last:expr) => { $last };
    ($first:expr $(, $rest:expr)*  $(, @$last:expr)? $(,)?) => { $crate::hlist::Cons { value: $first, rest: $crate::hlist!($($rest),* $(, @$last)?) } };
}

#[macro_export]
macro_rules! hlist_pat {
    () => { $crate::hlist::Nil };
    (, @$last:pat) => { $last };
    ($first:pat $(, $rest:pat)*  $(, @$last:pat)? $(,)?) => { $crate::hlist::Cons { value: $first, rest: $crate::hlist_pat!($($rest),* $(, @$last)?) } };
}

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

pub trait Access<T, N>: HList + Sized {
    type Remainder;

    fn take(self) -> (T, Self::Remainder);
    fn get(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
}

pub trait Split<T: HList, N>: HList + Sized {
    type Remainder;

    fn split(self) -> (T, Self::Remainder);
}

impl<T, R: HList> Access<T, peano::Zero> for Cons<T, R> {
    type Remainder = R;

    fn take(self) -> (T, Self::Remainder) { (self.value, self.rest) }

    fn get(&self) -> &T { &self.value }

    fn get_mut(&mut self) -> &mut T { &mut self.value }
}

impl<T, U, R: Access<U, N>, N> Access<U, peano::Succ<N>> for Cons<T, R> {
    type Remainder = Cons<T, R::Remainder>;

    fn take(self) -> (U, Self::Remainder) {
        let (value, rest) = self.rest.take();
        (value, hlist!(self.value, @rest))
    }

    fn get(&self) -> &U { self.rest.get() }

    fn get_mut(&mut self) -> &mut U { self.rest.get_mut() }
}

impl<T: HList> Split<Nil, TyList!()> for T {
    type Remainder = Self;

    fn split(self) -> (Nil, Self::Remainder) { (Nil, self) }
}

impl<T, U, R: HList, S: HList, N, M> Split<Cons<U, S>, TyList!(N, @M)> for Cons<T, R>
where
    Self: Access<U, N>,
    <Self as Access<U, N>>::Remainder: Split<S, M>,
{
    type Remainder = <<Self as Access<U, N>>::Remainder as Split<S, M>>::Remainder;

    fn split(self) -> (Cons<U, S>, Self::Remainder) {
        let (value, rest): (U, _) = self.take();
        let (rest, rem) = rest.split();
        (Cons { value, rest }, rem)
    }
}

impl<T: HList, U: HList, N> Shuffle<U, N> for T where T: Split<U, N, Remainder = Nil> {}
pub trait Shuffle<T: HList, N>: Split<T, N, Remainder = Nil> {
    fn shuffle(self) -> T {
        let (list, Nil) = self.split();
        list
    }
}
