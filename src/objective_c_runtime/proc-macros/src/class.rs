use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use syn::{parse_macro_input, FnArg, ImplItemMethod, ItemImpl, Pat};

#[derive(Debug)]
struct ExportAttributes {
    super_class_name: String,
    protocol: Option<String>,
}

impl ExportAttributes {
    pub fn new(attributes: &str) -> Self {
        let mut vals = attributes.split(',');

        Self {
            super_class_name: vals.next().unwrap().trim().to_string(),
            protocol: vals.next().map(|val| val.trim().to_string()),
        }
    }
}

pub fn register_class(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attributes = ExportAttributes::new(&attr.to_string());

    let input = parse_macro_input!(item as ItemImpl);
    let input_type = input.self_ty.clone();

    let super_class_name = Ident::new(&attributes.super_class_name, Span::call_site());
    let protocol_name = attributes
        .protocol
        .map(|val| Ident::new(&val, Span::call_site()));

    let protocol_block = if let Some(protocol_name) = protocol_name {
        quote! {
            {
            let p = Protocol::get(stringify!(#protocol_name)).unwrap();
            decl.add_protocol(p);
            }
        }
    } else {
        quote! {
            {}
        }
    };

    let methods = input
        .items
        .iter()
        .filter_map(|val| match val {
            syn::ImplItem::Method(val) => {
                if val.attrs.is_empty() {
                    return None;
                }

                let path = val.attrs[0].path.clone();
                if path.is_ident("selector_init") {
                    let add_method = quote! {
                        decl.add_method(
                            rust_macios::objective_c_runtime::sel!(init),
                            #input_type::generated_init as extern "C" fn(&Object, _) -> id,
                        );
                    };
                    Some(add_method)
                } else if path.is_ident("selector_impl") {
                    let objc_delegate = val.attrs[0].tokens.to_string();
                    let objc_delegate = get_objc_full_sel(&objc_delegate);
                    let sel_ts: proc_macro2::TokenStream = objc_delegate.parse().unwrap();
                    let fn_name = gen_sel_fn_name(val);
                    let fn_type = gen_sel_fn_type(val);

                    let add_method = quote! {
                        decl.add_method(
                            rust_macios::objective_c_runtime::sel!(#sel_ts),
                            #input_type::#fn_name as #fn_type,
                        );
                    };
                    Some(add_method)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    let dealloc_method = quote! {
        decl.add_method(
            rust_macios::objective_c_runtime::sel!(dealloc),
            #input_type::generated_dealloc as extern "C" fn(&rust_macios::objective_c_runtime::runtime::Object, _),
        );
    };

    let input_type_string = quote! {
        #input_type
    }
    .to_string();
    let objc_class_ident = format_ident!("RUST_{}", input_type_string);
    quote! {
        impl #input_type {
            pub fn register_class() -> &'static rust_macios::objective_c_runtime::runtime::Class {
                use std::sync::Once;

                use rust_macios::objective_c_runtime::{
                    class,
                    declare::ClassDecl,
                    msg_send,
                    runtime::{Class, Object, Protocol, Sel},
                    sel, sel_impl,
                };

                static INIT: Once = Once::new();

                INIT.call_once(|| unsafe {
                    let class_name = stringify!(#objc_class_ident);

                    let superclass = class!(#super_class_name);
                    let mut decl = ClassDecl::new(class_name, superclass).unwrap();

                    decl.add_ivar::<usize>("RUST_OBJ_PTR");

                    #protocol_block

                    #(#methods)*

                    #dealloc_method

                    decl.register();
                });

                Class::get(stringify!(#objc_class_ident)).unwrap()
            }

            pub fn init_objc_proxy_obj(self: std::sync::Arc<Self>) -> *mut rust_macios::objective_c_runtime::runtime::Object {
                use rust_macios::objective_c_runtime::{
                    class,
                    declare::ClassDecl,
                    msg_send,
                    runtime::{Class, Object, Protocol, Sel},
                    sel, sel_impl,
                };

                let class = #input_type::register_class();
                let objc_object = unsafe {
                    let ret: *mut rust_macios::objective_c_runtime::runtime::Object = msg_send![class, new];
                    ret
                };
                let raw_ptr = std::sync::Arc::into_raw(self);
                let raw_ptr_value = raw_ptr as usize;

                unsafe {
                    (&mut *objc_object).set_ivar("RUST_OBJ_PTR", raw_ptr_value);
                }

                objc_object
            }

            extern "C" fn generated_dealloc(this: &rust_macios::objective_c_runtime::runtime::Object, _: rust_macios::objective_c_runtime::runtime::Sel) {
                let arc = unsafe {
                    let raw_ptr_value: usize = *this.get_ivar("RUST_OBJ_PTR");
                    let raw_ptr = raw_ptr_value as *const Self;
                    std::sync::Arc::from_raw(raw_ptr)
                };
                drop(arc);
            }
        }

        impl PNSObject for #input_type {
            fn m_class<'a>() -> &'a rust_macios::objective_c_runtime::runtime::Class {
                Self::register_class()
            }

            fn m_self(&self) -> id {
                unsafe { msg_send![&*self.ptr, self] }
            }
        }


        unsafe impl rust_macios::objective_c_runtime::Message for #input_type { }

        #input
    }
    .into()
}

pub fn sel_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ImplItemMethod);
    let gen_sig = gen_sel_fn(&input);

    let method_name = input.sig.ident.clone();
    let args = gen_sel_fn_args(&input);
    let body = quote! {
        {
            let arc = unsafe {
                let raw_ptr_value: usize = *this.get_ivar("RUST_OBJ_PTR");
                let raw_ptr = raw_ptr_value as *const Self;
                std::sync::Arc::from_raw(raw_ptr)
            };

            arc.#method_name(#(#args,)*)
        }
    };

    quote! {
        #input

         #gen_sig
            #body
    }
    .into()
}

pub fn impl_init(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ImplItemMethod);

    let method_name = input.sig.ident.clone();

    quote! {
        #input

        extern "C" fn generated_init(this: &Object, _: rust_macios::objective_c_runtime::runtime::Sel) -> rust_macios::objective_c_runtime::id {
            use rust_macios::objective_c_runtime::{
                class, declare::ClassDecl,
                msg_send,
                runtime::{Class, Object, Protocol, Sel},
                sel, sel_impl,
            };

            let initialized_obj = Self::#method_name();

            let objc_object = unsafe {
                let ret: *mut rust_macios::objective_c_runtime::runtime::Object = msg_send![super(this, class!(NSObject)), init];
                ret
            };
            let arc = std::sync::Arc::new(initialized_obj);
            let raw_ptr = std::sync::Arc::into_raw(arc);
            let raw_ptr_value = raw_ptr as usize;

            unsafe {
                (&mut *objc_object).set_ivar("RUST_OBJ_PTR", raw_ptr_value);
            }

            objc_object
        }
    }.into()
}

fn get_objc_full_sel(token: &str) -> &str {
    token
        .trim_start_matches('(')
        .trim_start_matches('"')
        .trim_end_matches(')')
        .trim_end_matches('"')
}

fn gen_sel_fn(input: &ImplItemMethod) -> proc_macro2::TokenStream {
    let name = gen_sel_fn_name(input);

    let vis = input.vis.clone();
    let args: Vec<_> = input
        .sig
        .inputs
        .clone()
        .into_iter()
        .skip_while(|arg| match arg {
            FnArg::Receiver(_) => true,
            FnArg::Typed(_) => false,
        })
        .collect();
    let return_type = input.sig.output.clone();

    quote! {
        #vis extern "C" fn #name(this: &rust_macios::objective_c_runtime::runtime::Object, _sel: rust_macios::objective_c_runtime::runtime::Sel,  #(#args,)*) #return_type
    }
}

fn gen_sel_fn_type(input: &ImplItemMethod) -> proc_macro2::TokenStream {
    let args: Vec<_> = input
        .sig
        .inputs
        .clone()
        .into_iter()
        .skip_while(|arg| match arg {
            FnArg::Receiver(_) => true,
            FnArg::Typed(_) => false,
        })
        .collect();
    let return_type = input.sig.output.clone();

    quote! {
        extern "C" fn(this: &rust_macios::objective_c_runtime::runtime::Object, _sel: rust_macios::objective_c_runtime::runtime::Sel,  #(#args,)*) #return_type
    }
}

fn gen_sel_fn_name(input: &ImplItemMethod) -> proc_macro2::Ident {
    let name = input.sig.ident.to_string();
    let new_name = format!("generated_{name}");

    Ident::new(&new_name, Span::call_site())
}

fn gen_sel_fn_args(input: &ImplItemMethod) -> Vec<Pat> {
    input
        .sig
        .inputs
        .clone()
        .into_iter()
        .skip_while(|arg| match arg {
            FnArg::Receiver(_) => true,
            FnArg::Typed(_) => false,
        })
        .map(|val| match val {
            FnArg::Receiver(_) => unreachable!(),
            FnArg::Typed(val) => *val.pat,
        })
        .collect()
}
