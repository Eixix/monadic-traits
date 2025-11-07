use monadic_traits::maybe::Maybe::Just;
use monadic_traits::maybe::MaybeTC;
use monadic_traits::traits::{Applicative, Monad};

fn main() {
    let x = MaybeTC::bind(Just(10), |a| {
        MaybeTC::bind(Just(20), |b| MaybeTC::pure(a + b))
    });

    println!("{x:?}");

    let y = (|| {
        let a = Just(10)?;
        let b = Just(20)?;
        Just(a + b)
    })();

    println!("{y:?}");
}
