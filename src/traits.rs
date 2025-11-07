pub trait Functor {
    type Constructor<T>;

    fn fmap<A, B, F>(fa: Self::Constructor<A>, f: F) -> Self::Constructor<B>
    where
        F: FnOnce(A) -> B;
}

pub trait Applicative: Functor {
    fn pure<T>(x: T) -> Self::Constructor<T>;

    fn apply<A, B, F>(ff: Self::Constructor<F>, fa: Self::Constructor<A>) -> Self::Constructor<B>
    where
        F: FnOnce(A) -> B;
}

pub trait Monad: Applicative {
    fn bind<A, B, F>(ma: Self::Constructor<A>, f: F) -> Self::Constructor<B>
    where
        F: FnOnce(A) -> Self::Constructor<B>;
}
