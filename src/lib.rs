use std::fmt::Display;

pub use anerror_macros::catch;
pub use anerror_error::AnerrorPanic;

pub trait FallibleExt<T> {
    fn fail(self, msg: impl Display) -> T;
}

impl<T> FallibleExt<T> for Option<T> {
    fn fail(self, msg: impl Display) -> T {
        match self {
            Some(t) => t,
            None => panic!("{msg}"),
        }
    }
}

impl<T, E: Display> FallibleExt<T> for Result<T, E> {
    fn fail(self, msg: impl Display) -> T {
        match self {
            Ok(t) => t,
            Err(e) => panic!("{msg}: {e}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::FallibleExt;

    #[test]
    #[should_panic = "hey, it stopped working: a thing went wrong"]
    fn test_catch() {
        #[super::catch]
        fn t() {
            Result::<(), _>::Err("a thing went wrong").fail("hey, it stopped working");
        }

        t();
    }
}
