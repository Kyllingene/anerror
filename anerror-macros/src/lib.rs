use proc_macro::{TokenStream, Span};

use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn catch(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let sig = input.sig.clone();
    let block = input.block.clone();
    
    let mut inner_sig = input.sig.clone();

    TokenStream::from(quote::quote! {
        #sig {
            let result = std::panic::catch_unwind(|| {
                #block
            });

            match result {
                Ok(_) => return,
                Err(e) => {
                    if let Ok(e) = e.downcast::<crate::AnerrorPanic>() {
                        eprintln!("{e}");
                    } else {
                        eprintln!("todo: handle other panic things I guess?");
                    }

                    std::process::exit(1);
                }
            }
        }
    })
}
