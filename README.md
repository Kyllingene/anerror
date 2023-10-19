# Errata

Errata is a crate to simplify errors in your binary crates. The problem arises in code like this:

```rust
fn main() {
    let user_input = "abc";
    let number: i32 = user_input
        .parse()
        .expect("Invalid user input"); // this isn't viable in production
}
```

In this case, you would get a result like this:

```
thread 'main' panicked at src/main.rs:8:10:
Invalid user input: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

This is an ugly error. Unfortunately, your only alternative is to add supporting code to make sure that the input is valid, and print a useful error message if it isn't. This can not only get verbose, it's also difficult to get right if you want to call destructors; if you have open files with unflushed changes, those may get lost if you exit the program improperly.

## Enter errata

Errata gets it right so you don't have to. Using some unwinding magic (see below), errata catches your panics in a safety net to print them nicely. Since panics (usually) unwind, this calls all the destructors you need, but errata prints your errors nicely.

Here's the previous example with errata:

```rust
use errata::FallibleExt;

#[errata::catch]
fn main() {
    let user_input = "abc";
    let number: i32 = user_input
        .parse()
        .fail("Invalid user input"); // now, this prints a nice error message!
}
```

When this code fails, here's the output:

```
Invalid user input: invalid digit found in string
```

How nice! `fail` is also available for `Option`, with the minor difference that the section after and including the `:` is omitted (e.g. `Invalid user input`).

If you want to throw errors at arbitrary points, you may also use the `error!` macro, which is essentially a pretty-printed `panic!`.

## Normal panics

Normal panics, such as those caused by `unwrap`, `expect`, and `panic!`, are handled as well. It prints an error message akin to that produced by Rust, with location of error as well as an optional backtrace. This ensures that unexpected errors still give you useful information.

## Color

There is also basic color support in the forms of `fail_color` and `error_color!`, both of which print their error messages in bold red. If more diverse color support is something you want to see, feel free to submit a PR or an issue.

## How it works

Under the hood, errata wraps your code in [`catch_unwind`](https://doc.rust_lang.org/std/panic/fn.catch_unwind.html), which just means that it can catch panics and print them nicely before exiting.

Since it was ran inside `catch_unwind`, your destructors will be called and your error will be caught. This is accomplished by overriding the default panic handler (to suppress panic messages), handling the error slightly differently depending on its type.

Unfortunately, due to a lack of information regarding the type of a panic payload, not every panic can be neatly handled. However, 99% of the time you will be dealing with `String` or `&str` payloads, which *are* handled neatly.

