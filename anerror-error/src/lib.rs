use std::fmt::Display;

#[doc(hidden)]
#[derive(Debug)]
pub struct AnerrorPanic(Box<dyn std::error::Error>);

impl Display for AnerrorPanic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
