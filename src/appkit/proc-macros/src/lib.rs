use proc_macro::TokenStream;

mod ns_application_main;

#[proc_macro_attribute]
pub fn ns_application_main(args: TokenStream, item: TokenStream) -> TokenStream {
    ns_application_main::entry_point(item)
}
