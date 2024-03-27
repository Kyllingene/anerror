use errata::{FatalError, WithContext};

fn foo() -> Result<i32, u64> {
    Err(2)
}

fn main() -> FatalError<(), String> {
    let bar = foo().context("foo failed")?;
    println!("{bar}");
    ().into()
}
