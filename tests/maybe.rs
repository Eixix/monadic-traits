use monadic_traits::maybe::Maybe;
use monadic_traits::maybe::Maybe::{Just, Nothing};

fn try_maybe() -> Maybe<i32> {
    let a = Just(10)?;
    let b = Just(20)?;
    Just(a + b)
}

fn try_maybe_fail() -> Maybe<i32> {
    let a = Just(10)?;
    let _b = Nothing?; // Should fail
    Just(a)
}

#[test]
fn test_try_success() {
    assert_eq!(try_maybe(), Just(30));
}

#[test]
fn test_try_failure() {
    assert_eq!(try_maybe_fail(), Nothing);
}