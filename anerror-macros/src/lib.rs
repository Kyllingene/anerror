use proc_macro::TokenStream;

use syn::{parse_macro_input, ItemFn};

/// Sets up nice error handling for your Rust function.
///
/// Technically, this can be applied to any function. However, you only need
/// to apply it to your `main` function. It should never be used in libraries
/// (that applies to the whole crate).
///
/// If you forget this, your pretty errors will all show up as `Box<dyn Any>`.
#[proc_macro_attribute]
pub fn catch(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let sig = input.sig;
    let block = input.block;

    TokenStream::from(quote::quote! {
        #sig {
            let old_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));

            let result = std::panic::catch_unwind(|| {
                #block
            });

            match result {
                Ok(_) => return,
                Err(e) => {
                    if let Some(e) = e.downcast_ref::<anerror::AnerrorPanic>() {
                        eprintln!("{e}");
                    } else if let Some(e) = e.downcast_ref::<&str>() {
                        eprintln!("error (at {}:{}:{}): {e}\n{}", file!(), line!(), column!(), std::backtrace::Backtrace::capture());
                    } else if let Some(e) = e.downcast_ref::<String>() {
                        eprintln!("error (at {}:{}:{}): {e}", file!(), line!(), column!());

                        let bt = std::backtrace::Backtrace::capture();

                        use std::backtrace::BacktraceStatus as BtS;
                        match bt.status() {
                            BtS::Captured => eprintln!("{bt}"),
                            BtS::Disabled => eprintln!("run with `RUST_BACKTRACE=1` environment variable to display a backtrace"),
                            _ => {}
                        }
                    } else {
                        eprintln!("Unhandled error in {} at line {}:{}\nErrored", file!(), line!(), column!());
                    }

                    std::process::exit(1);
                }
            }
        }
    })
}
