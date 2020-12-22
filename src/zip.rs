use crate::hlist::{Cons, Nil};

pub type Zipped<T, U> = <T as Zip<U>>::Output;
pub trait Zip<O> {
    type Output;

    fn zip(self, other: O) -> Self::Output;
}

impl Zip<Nil> for Nil {
    type Output = Self;

    fn zip(self, Nil: Self) -> Self::Output { Self }
}

impl<T, U, R: Zip<S>, S> Zip<Cons<U, S>> for Cons<T, R> {
    type Output = Cons<(T, U), R::Output>;

    fn zip(self, other: Cons<U, S>) -> Self::Output {
        Cons {
            value: (self.value, other.value),
            rest: self.rest.zip(other.rest),
        }
    }
}
