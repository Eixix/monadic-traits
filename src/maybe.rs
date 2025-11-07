use crate::traits::{Applicative, Functor, Monad};
use std::ops::{ControlFlow, FromResidual, Try};

#[derive(Debug, PartialEq)]
pub enum Maybe<T> {
    Just(T),
    Nothing,
}

pub struct MaybeTC;

impl Functor for MaybeTC {
    type Constructor<T> = Maybe<T>;

    fn fmap<A, B, F>(fa: Self::Constructor<A>, f: F) -> Self::Constructor<B>
    where
        F: FnOnce(A) -> B,
    {
        match fa {
            Maybe::Just(a) => Maybe::Just(f(a)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }
}

impl Applicative for MaybeTC {
    fn pure<T>(x: T) -> Self::Constructor<T> {
        Maybe::Just(x)
    }

    fn apply<A, B, F>(ff: Self::Constructor<F>, fa: Self::Constructor<A>) -> Self::Constructor<B>
    where
        F: FnOnce(A) -> B,
    {
        match (ff, fa) {
            (Maybe::Just(f), Maybe::Just(a)) => Maybe::Just(f(a)),
            _ => Maybe::Nothing,
        }
    }
}

impl Monad for MaybeTC {
    fn bind<A, B, F>(ma: Self::Constructor<A>, f: F) -> Self::Constructor<B>
    where
        F: FnOnce(A) -> Self::Constructor<B>,
    {
        match ma {
            Maybe::Just(a) => f(a),
            Maybe::Nothing => Maybe::Nothing,
        }
    }
}

impl<T> FromResidual<Maybe<!>> for Maybe<T> {
    fn from_residual(_: Maybe<!>) -> Self {
        Maybe::Nothing
    }
}

impl<T> Try for Maybe<T>
where
    Maybe<T>: FromResidual<Maybe<!>>,
{
    type Output = T;
    type Residual = Maybe<!>;

    fn from_output(output: Self::Output) -> Self {
        Maybe::Just(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Maybe::Just(t) => ControlFlow::Continue(t),
            Maybe::Nothing => ControlFlow::Break(Maybe::Nothing),
        }
    }
}
