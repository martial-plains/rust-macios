use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, FnArg, ImplItemMethod, ItemImpl, Signature};

#[derive(Debug)]
struct ExportAttributes {
    super_class_name: String,
    is_super_class_generic: bool,
}

impl ExportAttributes {
    pub fn new(attributes: &str) -> Self {
        let mut vals = attributes.split(',');
        let is_super_class_generic = if attributes.contains('<') {
            let left_angle_count = attributes.chars().filter(|c| *c == '<').count();
            let right_angle_count = attributes.chars().filter(|c| *c == '>').count();

            match left_angle_count.cmp(&right_angle_count) {
                std::cmp::Ordering::Less => {
                    quote_spanned! {
                        attributes.span() =>
                        compile_error!("There is an extra '>'\n");
                    };
                    false
                }
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Greater => {
                    quote_spanned! {
                        attributes.span() =>
                        compile_error!("There is an extra '<'\n");
                    };
                    false
                }
            }
        } else {
            false
        };

        Self {
            super_class_name: vals.next().unwrap().trim().to_string(),
            is_super_class_generic,
        }
    }
}

pub fn interface_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attributes = ExportAttributes::new(&attr.to_string());

    let input = parse_macro_input!(item as ItemImpl);
    let input_type = input.self_ty.clone();
    let input_type = quote!(#input_type);

    let super_class_name = Ident::new(&attributes.super_class_name, Span::call_site());

    let methods = input
        .items
        .iter()
        .filter_map(|val| match val {
            syn::ImplItem::Method(val) => {
                if val.attrs.is_empty() {
                    return None;
                }

                if val.attrs.iter().any(|a| a.path.is_ident("method")) {
                    Some(method_sig(val))
                } else if val.attrs.iter().any(|a| a.path.is_ident("property")) {
                    Some(property_sig(val))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    // Create the [`trait`] and type's impl block
    let type_impl = quote! {
        impl #input_type {
            #(#methods)*
        }
    };

    let type_trait = {
        let super_class_name = super_class_name.to_string();
        let sup_name = if &super_class_name == "NSObject" {
            Ident::new("PNSObject", Span::call_site())
        } else {
            Ident::new(&format!("I{super_class_name}"), Span::call_site())
        };

        let name = {
            let input_type = quote!(#input_type).to_string();

            Ident::new(&format!("I{input_type}"), Span::call_site())
        };

        let trait_fns = input
            .items
            .iter()
            .filter_map(|val| match val {
                syn::ImplItem::Method(val) => {
                    if val.attrs.is_empty() {
                        return None;
                    }

                    if val.attrs.iter().any(|a| a.path.is_ident("method")) {
                        Some(trait_method_sig(val))
                    } else if val.attrs.iter().any(|a| a.path.is_ident("property")) {
                        Some(trait_property_sig(val))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        match input.generics.params.is_empty() {
            true => quote! {
                pub trait #name : #sup_name {
                    #(#trait_fns)*
                }
            },
            false => {
                let generic_params = input.generics.params.iter();
                match attributes.is_super_class_generic {
                    true => {
                        let sup_generic_params = input.generics.params.iter();

                        quote! {
                            pub trait #name<#(#generic_params),*> : #sup_name<#(#sup_generic_params),*> {
                                #(#trait_fns)*
                            }
                        }
                    }
                    false => quote! {
                        pub trait #name<#(#generic_params),*> : #sup_name {
                            #(#trait_fns)*
                        }
                    },
                }
            }
        }
    };

    let trait_name = {
        let input_type = quote!(#input_type).to_string();

        Ident::new(&format!("I{input_type}"), Span::call_site())
    };

    let trait_doc = format!("A trait containing all the methods for [`{input_type}`]");

    match input.generics.params.is_empty() {
        true => quote! {
            #type_impl

            #[doc = #trait_doc]
            #type_trait

            impl #trait_name for #input_type {}
        }
        .into(),
        false => {
            let generic_params = input.generics.params.iter();
            let input_type_generic = generic_params.clone();

            match input.generics.where_clause {
                Some(clause) => quote! {
                    #type_impl

                    #[doc = #trait_doc]
                    #type_trait

                    impl<#(#generic_params),*> #trait_name for #input_type<#(#input_type_generic),*> #clause {}
                }
                .into(),
                None => todo!(),
            }
        }
    }
}

/// This method is for extracting the method data from the [`ImplItemMethod`] and parsing a new
/// method replacing the body to contain a method call to the Interface's trait method which matches the name of the method
/// excluding the prefix "m_".
fn method_sig(input: &ImplItemMethod) -> proc_macro2::TokenStream {
    let name = input.sig.ident.clone();
    let return_type = input.sig.output.clone();

    let trait_name = Ident::new(&format!("m_{name}"), Span::call_site());

    let generics = input.sig.generics.clone();

    let where_clause = generics.where_clause;

    let generic_params = generics.params;

    let mut fn_args: Vec<FnArg> = vec![];

    if has_arguments(input.sig.clone()) {
        fn_args = input
            .sig
            .inputs
            .iter()
            .skip_while(|arg| match arg {
                FnArg::Receiver(_) => true,
                FnArg::Typed(_) => false,
            })
            .cloned()
            .collect();
    }

    let arg_name = fn_args
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => unreachable!(),
            FnArg::Typed(pat) => *pat.pat.clone(),
        })
        .collect::<Vec<_>>();

    let arg_type = fn_args
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => unreachable!(),
            FnArg::Typed(pat) => *pat.ty.clone(),
        })
        .collect::<Vec<_>>();

    let attrs = input
        .attrs
        .iter()
        .filter(|attr| !attr.path.is_ident("method"))
        .collect::<Vec<_>>();

    match input.sig.receiver() {
        Some(receiver) => match receiver {
            syn::FnArg::Receiver(receiver) => match receiver.mutability {
                Some(_) => match generic_params.is_empty() {
                    true => match where_clause {
                        Some(clause) => quote! {
                            #(#attrs)*
                            pub fn #name(&mut self, #(#arg_name: #arg_type),* ) #return_type #clause {
                                self.#trait_name(#(#arg_name),*)
                            }
                        },
                        None => quote! {
                            #(#attrs)*
                            pub fn #name(&mut self, #(#arg_name: #arg_type),* ) #return_type {
                                self.#trait_name(#(#arg_name),*)
                            }
                        },
                    },
                    false => {
                        let generic_params = generic_params.iter();

                        match where_clause {
                            Some(clause) => quote! {
                                #(#attrs)*
                                pub fn #name<#(#generic_params),*>(&mut self, #(#arg_name: #arg_type),* ) #return_type #clause {
                                    self.#trait_name(#(#arg_name),*)
                                }
                            },
                            None => quote! {
                                #(#attrs)*
                                pub fn #name<#(#generic_params),*>(&mut self, #(#arg_name: #arg_type),* ) #return_type {
                                    self.#trait_name(#(#arg_name),*)
                                }
                            },
                        }
                    }
                },
                None => match generic_params.is_empty() {
                    true => match where_clause {
                        Some(clause) => quote! {
                            #(#attrs)*
                            pub fn #name(&self, #(#arg_name : #arg_type),*) #return_type #clause {
                                self.#trait_name(#(#arg_name),*)
                            }
                        },
                        None => quote! {
                            #(#attrs)*
                            pub fn #name(&self, #(#arg_name : #arg_type),*) #return_type {
                                self.#trait_name(#(#arg_name),*)
                            }
                        },
                    },
                    false => {
                        let generic_params = generic_params.iter();

                        match where_clause {
                            Some(clause) => quote! {
                                #(#attrs)*
                                pub fn #name<#(#generic_params),*>(&self, #(#arg_name : #arg_type),*) #return_type #clause {
                                    self.#trait_name(#(#arg_name),*)
                                }
                            },
                            None => quote! {
                                #(#attrs)*
                                pub fn #name<#(#generic_params),*>(&self, #(#arg_name : #arg_type),*) #return_type {
                                    self.#trait_name(#(#arg_name),*)
                                }
                            },
                        }
                    }
                },
            },
            syn::FnArg::Typed(_) => unreachable!(),
        },
        None => match generic_params.is_empty() {
            true => quote! {
                #(#attrs)*
                pub fn #name(#(#arg_name : #arg_type,)*) #return_type {
                    Self::#trait_name(#(#arg_name),*)
                }
            },
            false => {
                let generic_params = generic_params.iter();

                match where_clause {
                    Some(clause) => quote! {
                        #(#attrs)*
                        pub fn #name<#(#generic_params),*>(#(#arg_name : #arg_type,)*) #return_type #clause {
                            Self::#trait_name(#(#arg_name),*)
                        }
                    },
                    None => quote! {
                        #(#attrs)*
                        pub fn #name<#(#generic_params),*>(#(#arg_name : #arg_type,)*) #return_type {
                            Self::#trait_name(#(#arg_name),*)
                        }
                    },
                }
            }
        },
    }
}

/// This method is for extracting the method data from the [`ImplItemMethod`] and parsing a new
/// trait function which contains the statements from the given [`ImplItemMethod`] argument.
fn trait_method_sig(input: &ImplItemMethod) -> proc_macro2::TokenStream {
    let name = Ident::new(&format!("m_{}", input.sig.clone().ident), Span::call_site());
    let return_type = input.sig.output.clone();

    let generics = input.sig.generics.clone();

    let where_clause = generics.where_clause;

    let generic_params = generics.params;

    let body = input.block.stmts.clone();

    let mut fn_args: Vec<FnArg> = vec![];

    if has_arguments(input.sig.clone()) {
        fn_args = input
            .sig
            .inputs
            .iter()
            .skip_while(|arg| match arg {
                FnArg::Receiver(_) => true,
                FnArg::Typed(_) => false,
            })
            .cloned()
            .collect();
    }

    let arg_name = fn_args
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => unreachable!(),
            FnArg::Typed(pat) => *pat.pat.clone(),
        })
        .collect::<Vec<_>>();

    let arg_type = fn_args
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => unreachable!(),
            FnArg::Typed(pat) => *pat.ty.clone(),
        })
        .collect::<Vec<_>>();

    let attrs = input
        .attrs
        .iter()
        .filter(|attr| !attr.path.is_ident("method"))
        .collect::<Vec<_>>();

    match input.sig.receiver() {
        Some(receiver) => match receiver {
            syn::FnArg::Receiver(receiver) => match receiver.mutability {
                Some(_) => match generic_params.is_empty() {
                    true => match where_clause {
                        Some(clause) => quote! {
                            #(#attrs)*
                            fn #name(&mut self, #(#arg_name: #arg_type),* ) #return_type #clause {
                                #(#body)*
                            }
                        },
                        None => quote! {
                            #(#attrs)*
                            fn #name(&mut self, #(#arg_name : #arg_type),* ) #return_type {
                                #(#body)*
                            }
                        },
                    },
                    false => {
                        let generic_params = generic_params.iter();
                        match where_clause {
                            Some(clause) => quote! {
                                #(#attrs)*
                                fn #name<#(#generic_params),*>(&mut self, #(#arg_name: #arg_type),* ) #return_type #clause {
                                    #(#body)*
                                }
                            },
                            None => quote! {
                                #(#attrs)*
                                fn #name<#(#generic_params),*>(&mut self, #(#arg_name : #arg_type),* ) #return_type {
                                    #(#body)*
                                }
                            },
                        }
                    }
                },
                None => match generic_params.is_empty() {
                    true => match where_clause {
                        Some(clause) => quote! {
                            #(#attrs)*
                            fn #name(&self, #(#arg_name : #arg_type),*) #return_type #clause {
                               #(#body)*
                            }
                        },
                        None => quote! {
                            #(#attrs)*
                            fn #name(&self, #(#arg_name : #arg_type),*) #return_type {
                               #(#body)*
                            }
                        },
                    },
                    false => {
                        let generic_params = generic_params.iter();

                        match where_clause {
                            Some(clause) => quote! {
                                #(#attrs)*
                                fn #name<#(#generic_params),*>(&self, #(#arg_name : #arg_type),*) #return_type #clause {
                                   #(#body)*
                                }
                            },
                            None => quote! {
                                #(#attrs)*
                                fn #name<#(#generic_params),*>(&self, #(#arg_name : #arg_type),*) #return_type {
                                   #(#body)*
                                }
                            },
                        }
                    }
                },
            },
            syn::FnArg::Typed(_) => unreachable!(),
        },
        None => match generic_params.is_empty() {
            true => match where_clause {
                Some(clause) => quote! {
                    #(#attrs)*
                    fn #name(#(#arg_name: #arg_type),*) #return_type #clause {
                        #(#body)*
                    }
                },
                None => quote! {
                    #(#attrs)*
                    fn #name(#(#arg_name: #arg_type),*) #return_type {
                        #(#body)*
                    }
                },
            },
            false => {
                let generic_params = generic_params.iter();

                match where_clause {
                    Some(clause) => quote! {
                        #(#attrs)*
                        fn #name<#(#generic_params),*>(#(#arg_name: #arg_type),*) #return_type #clause {
                            #(#body)*
                        }
                    },
                    None => quote! {
                        #(#attrs)*
                        fn #name<#(#generic_params),*>(#(#arg_name: #arg_type),*) #return_type {
                            #(#body)*
                        }
                    },
                }
            }
        },
    }
}

/// This method is for extracting the method data from the [`ImplItemMethod`] and parsing a new
/// method replacing the body to contain a method call to the Interface's trait method which matches the name of the method
/// excluding the prefix "m_".
fn property_sig(input: &ImplItemMethod) -> proc_macro2::TokenStream {
    let name = input.sig.ident.clone();
    let return_type = input.sig.output.clone();

    let trait_name = Ident::new(&format!("p_{name}"), Span::call_site());

    let generics = input.sig.generics.clone();

    let where_clause = generics.where_clause;

    let generic_params = generics.params;

    let mut fn_args: Vec<FnArg> = vec![];

    if has_arguments(input.sig.clone()) {
        fn_args = input
            .sig
            .inputs
            .iter()
            .skip_while(|arg| match arg {
                FnArg::Receiver(_) => true,
                FnArg::Typed(_) => false,
            })
            .cloned()
            .collect();
    }

    let arg_name = fn_args
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => unreachable!(),
            FnArg::Typed(pat) => *pat.pat.clone(),
        })
        .collect::<Vec<_>>();

    let arg_type = fn_args
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => unreachable!(),
            FnArg::Typed(pat) => *pat.ty.clone(),
        })
        .collect::<Vec<_>>();

    let attrs = input
        .attrs
        .iter()
        .filter(|attr| !attr.path.is_ident("property"))
        .collect::<Vec<_>>();

    match input.sig.receiver() {
        Some(receiver) => match receiver {
            syn::FnArg::Receiver(receiver) => match receiver.mutability {
                Some(_) => match generic_params.is_empty() {
                    true => match where_clause {
                        Some(clause) => quote! {
                            #(#attrs)*
                            pub fn #name(&mut self, #(#arg_name: #arg_type),* ) #return_type #clause {
                                self.#trait_name(#(#arg_name),*)
                            }
                        },
                        None => quote! {
                            #(#attrs)*
                            pub fn #name(&mut self, #(#arg_name: #arg_type),* ) #return_type {
                                self.#trait_name(#(#arg_name),*)
                            }
                        },
                    },
                    false => {
                        let generic_params = generic_params.iter();

                        match where_clause {
                            Some(clause) => quote! {
                                #(#attrs)*
                                pub fn #name<#(#generic_params),*>(&mut self, #(#arg_name: #arg_type),* ) #return_type #clause {
                                    self.#trait_name(#(#arg_name),*)
                                }
                            },
                            None => quote! {
                                #(#attrs)*
                                pub fn #name<#(#generic_params),*>(&mut self, #(#arg_name: #arg_type),* ) #return_type {
                                    self.#trait_name(#(#arg_name),*)
                                }
                            },
                        }
                    }
                },
                None => match generic_params.is_empty() {
                    true => match where_clause {
                        Some(clause) => quote! {
                            #(#attrs)*
                            pub fn #name(&self, #(#arg_name : #arg_type),*) #return_type #clause {
                                self.#trait_name(#(#arg_name),*)
                            }
                        },
                        None => quote! {
                            #(#attrs)*
                            pub fn #name(&self, #(#arg_name : #arg_type),*) #return_type {
                                self.#trait_name(#(#arg_name),*)
                            }
                        },
                    },
                    false => {
                        let generic_params = generic_params.iter();

                        match where_clause {
                            Some(clause) => quote! {
                                #(#attrs)*
                                pub fn #name<#(#generic_params),*>(&self, #(#arg_name : #arg_type),*) #return_type #clause {
                                    self.#trait_name(#(#arg_name),*)
                                }
                            },
                            None => quote! {
                                #(#attrs)*
                                pub fn #name<#(#generic_params),*>(&self, #(#arg_name : #arg_type),*) #return_type {
                                    self.#trait_name(#(#arg_name),*)
                                }
                            },
                        }
                    }
                },
            },
            syn::FnArg::Typed(_) => unreachable!(),
        },
        None => quote! {
            #(#attrs)*
            pub fn #name(#(#arg_name : #arg_type,)*) #return_type {
                Self::#trait_name(#(#arg_name),*)
            }
        },
    }
}

/// This method is for extracting the method data from the [`ImplItemMethod`] and parsing a new
/// trait function which contains the statements from the given [`ImplItemMethod`] argument.
fn trait_property_sig(input: &ImplItemMethod) -> proc_macro2::TokenStream {
    let name = Ident::new(&format!("p_{}", input.sig.clone().ident), Span::call_site());
    let return_type = input.sig.output.clone();

    let generics = input.sig.generics.clone();

    let where_clause = generics.where_clause;

    let generic_params = generics.params;

    let body = input.block.stmts.clone();

    let mut fn_args: Vec<FnArg> = vec![];

    if has_arguments(input.sig.clone()) {
        fn_args = input
            .sig
            .inputs
            .iter()
            .skip_while(|arg| match arg {
                FnArg::Receiver(_) => true,
                FnArg::Typed(_) => false,
            })
            .cloned()
            .collect();
    }

    let arg_name = fn_args
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => unreachable!(),
            FnArg::Typed(pat) => *pat.pat.clone(),
        })
        .collect::<Vec<_>>();

    let arg_type = fn_args
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => unreachable!(),
            FnArg::Typed(pat) => *pat.ty.clone(),
        })
        .collect::<Vec<_>>();

    let attrs = input
        .attrs
        .iter()
        .filter(|attr| !attr.path.is_ident("property"))
        .collect::<Vec<_>>();

    match input.sig.receiver() {
        Some(receiver) => match receiver {
            syn::FnArg::Receiver(receiver) => match receiver.mutability {
                Some(_) => match generic_params.is_empty() {
                    true => match where_clause {
                        Some(clause) => quote! {
                            #(#attrs)*
                            fn #name(&mut self, #(#arg_name: #arg_type),* ) #return_type #clause {
                                #(#body)*
                            }
                        },
                        None => quote! {
                            #(#attrs)*
                            fn #name(&mut self, #(#arg_name : #arg_type),* ) #return_type {
                                #(#body)*
                            }
                        },
                    },
                    false => {
                        let generic_params = generic_params.iter();
                        match where_clause {
                            Some(clause) => quote! {
                                #(#attrs)*
                                fn #name<#(#generic_params),*>(&mut self, #(#arg_name: #arg_type),* ) #return_type #clause {
                                    #(#body)*
                                }
                            },
                            None => quote! {
                                #(#attrs)*
                                fn #name<#(#generic_params),*>(&mut self, #(#arg_name : #arg_type),* ) #return_type {
                                    #(#body)*
                                }
                            },
                        }
                    }
                },
                None => match generic_params.is_empty() {
                    true => match where_clause {
                        Some(clause) => quote! {
                            #(#attrs)*
                            fn #name(&self, #(#arg_name : #arg_type),*) #return_type #clause {
                               #(#body)*
                            }
                        },
                        None => quote! {
                            #(#attrs)*
                            fn #name(&self, #(#arg_name : #arg_type),*) #return_type {
                               #(#body)*
                            }
                        },
                    },
                    false => {
                        let generic_params = generic_params.iter();

                        match where_clause {
                            Some(clause) => quote! {
                                #(#attrs)*
                                fn #name<#(#generic_params),*>(&self, #(#arg_name : #arg_type),*) #return_type #clause {
                                   #(#body)*
                                }
                            },
                            None => quote! {
                                #(#attrs)*
                                fn #name<#(#generic_params),*>(&self, #(#arg_name : #arg_type),*) #return_type {
                                   #(#body)*
                                }
                            },
                        }
                    }
                },
            },
            syn::FnArg::Typed(_) => unreachable!(),
        },
        None => match generic_params.is_empty() {
            true => match where_clause {
                Some(clause) => quote! {
                    #(#attrs)*
                    fn #name(#(#arg_name: #arg_type),*) #return_type #clause {
                        #(#body)*
                    }
                },
                None => quote! {
                    #(#attrs)*
                    fn #name(#(#arg_name: #arg_type),*) #return_type {
                        #(#body)*
                    }
                },
            },
            false => {
                let generic_params = generic_params.iter();

                match where_clause {
                    Some(clause) => quote! {
                        #(#attrs)*
                        fn #name<#(#generic_params),*>(#(#arg_name: #arg_type),*) #return_type #clause {
                            #(#body)*
                        }
                    },
                    None => quote! {
                        #(#attrs)*
                        fn #name<#(#generic_params),*>(#(#arg_name: #arg_type),*) #return_type {
                            #(#body)*
                        }
                    },
                }
            }
        },
    }
}

fn has_arguments(args: Signature) -> bool {
    (args.receiver().is_some() && args.inputs.len() > 1)
        || (args.receiver().is_none() && !args.inputs.is_empty())
}
