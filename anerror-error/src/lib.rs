use std::fmt::Display;

#[doc(hidden)]
#[derive(Debug)]
pub struct AnerrorPanic(pub String);

impl Display for AnerrorPanic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

