use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn entry_point(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);

    let stms = item.block.stmts;

    let usings = quote! {
        use rust_macios::appkit::NSApplicationMain;
        use std::os::raw::c_int;
    };

    let user_code = quote! {
        #(#stms)*
    };

    let main_func = quote! {
        #usings
        fn main() {
            #user_code
            unsafe {
                let args = std::env::args();
                let argv = &args as *const _ as *const *const i8;
                NSApplicationMain(args.len() as c_int, argv);
            }
        }
    };

    quote! {
        #main_func
    }
    .into()
}
