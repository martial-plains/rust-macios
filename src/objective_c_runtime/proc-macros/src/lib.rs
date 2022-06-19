use proc_macro::TokenStream;

extern crate proc_macro;

mod class;

#[proc_macro_attribute]
pub fn objc_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    class::objc_impl(attr, item)
}

#[proc_macro_attribute]
pub fn objc_selector_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    class::sel_impl(attr, item)
}

#[proc_macro_attribute]
pub fn objc_impl_init(_attr: TokenStream, item: TokenStream) -> TokenStream {
    class::impl_init(_attr, item)
}
