use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn entry_point(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);

    let block = *item.block;

    quote! {
        use rust_macios::appkit::NSApplicationMain;
        use std::os::raw::c_int;

        #[allow(dead_code)]
        fn main() {
            unsafe {
                #block
                let args = std::env::args();
                let argv = &args as *const _ as *const *const i8;
                NSApplicationMain(args.len() as c_int, argv);
            }
        }
    }
    .into()
}
