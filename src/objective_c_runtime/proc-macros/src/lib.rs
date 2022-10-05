use proc_macro::TokenStream;

extern crate proc_macro;

mod class;
mod interface;

#[proc_macro_attribute]
pub fn register_class(attr: TokenStream, item: TokenStream) -> TokenStream {
    class::register_class(attr, item)
}

#[proc_macro_attribute]
pub fn objc_sel(attr: TokenStream, item: TokenStream) -> TokenStream {
    class::sel_impl(attr, item)
}

#[proc_macro_attribute]
pub fn class_init(_attr: TokenStream, item: TokenStream) -> TokenStream {
    class::impl_init(_attr, item)
}

#[proc_macro_attribute]
pub fn interface_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    interface::interface_impl(attr, item)
}
