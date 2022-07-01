/// The given name must be a valid Objective-C class that inherits NSObject.
pub(crate) macro object {
    (
        $(#[$m:meta])*
        unsafe $v:vis struct $name:ident $(;)?
    ) => {
        object! {
            $(#[$m])*
            unsafe $v struct $name<> {,}
        }
    },
    (
        $(#[$m:meta])*
        unsafe $v:vis struct $name:ident<$($t:ident $(: $b:ident)?),*> {
            $($p:ident: $pty:ty),*$(,)+
        }
    ) => {
        $(#[$m])*
        #[repr(C)]
        $v struct $name<$($t $(: $b)?),*> {
            /// The raw pointer to the Objective-C object.
            pub ptr: $crate::objective_c_runtime::Id<$crate::objective_c_runtime::runtime::Object>,
            // Additional fields (should only be zero-sized PhantomData).
            $($p : $pty,)*
        }

        impl<$($t $(: $b)?),*> $crate::objective_c_runtime::traits::PNSObject for $name<$($t),*> {
            fn im_class<'a>() -> &'a $crate::objective_c_runtime::runtime::Class {
                $crate::objective_c_runtime::class!($name)
            }

            fn im_self(&self) -> $crate::objective_c_runtime::id {
                use $crate::objective_c_runtime::{msg_send, sel, sel_impl};
                unsafe { msg_send![&*self.ptr, self] }
            }
        }

        impl<$($t: core::hash::Hash $(+ $b)?),*> ::core::hash::Hash for $name<$($t),*> {
            #[inline]
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                use $crate::objective_c_runtime::traits::PNSObject;
                self.ip_hash().hash(state);
            }
        }

        impl<$($t $(: $b)?),*> core::fmt::Debug for $name<$($t),*> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                use $crate::objective_c_runtime::traits::PNSObject;
                std::write!(f, "{}", self.ip_debug_description())
            }
        }

        impl<$($t $(: $b)?),*> core::fmt::Display for $name<$($t),*> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                use $crate::objective_c_runtime::traits::PNSObject;
                std::write!(f, "{}", self.ip_description())
            }
        }

        impl<$($t $(: $b)?),*> $crate::objective_c_runtime::traits::ToId for $name<$($t),*> {
            fn to_id(mut self) -> $crate::objective_c_runtime::id {
                &mut *self.ptr
            }
        }

        impl<$($t $(: $b)?),*> $crate::objective_c_runtime::traits::FromId for $name<$($t),*> {
            unsafe fn from_id(ptr: $crate::objective_c_runtime::id) -> Self {
                Self {
                    ptr: crate::objective_c_runtime::Id::from_ptr(ptr),
                     $($p: std::marker::PhantomData),*
                }
            }
        }
    },
}

/// The given name must be a valid Objective-C class that inherits NSObject.
pub(crate) macro shared_object {
    (
        $(#[$m:meta])*
        unsafe $v:vis struct $name:ident<$($t:ident $(: $b:ident)?),*> {
            $($p:ident: $pty:ty),*$(,)+
        }
    ) => {
        $(#[$m])*
        #[repr(C)]
        $v struct $name<$($t $(: $b)?),*> {
            /// The raw pointer to the Objective-C object.
            pub ptr: $crate::objective_c_runtime::ShareId<$crate::objective_c_runtime::runtime::Object>,
            // Additional fields (should only be zero-sized PhantomData).
            $($p: $pty),*
        }

        impl<$($t $(: $b)?),*> $crate::objective_c_runtime::traits::PNSObject for $name<$($t),*> {
            fn im_class<'a>() -> &'a $crate::objective_c_runtime::runtime::Class {
                $crate::objective_c_runtime::class!($name)
            }

            fn im_self(&self) -> $crate::objective_c_runtime::id {
                use $crate::objective_c_runtime::{msg_send, sel, sel_impl};
                unsafe { msg_send![&*self.ptr, self] }
            }
        }

        impl<$($t: core::hash::Hash $(+ $b)?),*> ::core::hash::Hash for $name<$($t),*> {
            #[inline]
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                use $crate::objective_c_runtime::traits::PNSObject;
                self.ip_hash().hash(state);
            }
        }

        impl<$($t $(: $b)?),*> core::fmt::Debug for $name<$($t),*> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                use $crate::objective_c_runtime::traits::PNSObject;
                std::write!(f, "{}", self.ip_debug_description())
            }
        }

        impl<$($t $(: $b)?),*> core::fmt::Display for $name<$($t),*> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                use $crate::objective_c_runtime::traits::PNSObject;
                std::write!(f, "{}", self.ip_description())
            }
        }

        impl<$($t $(: $b)?),*> $crate::objective_c_runtime::traits::ToId for $name<$($t),*> {
            fn to_id(self) -> $crate::objective_c_runtime::id {
                use $crate::objective_c_runtime::traits::PNSObject;
                self.im_self()
            }
        }

        impl<$($t $(: $b)?),*> $crate::objective_c_runtime::traits::FromId for $name<$($t),*> {
            unsafe fn from_id(ptr: $crate::objective_c_runtime::id) -> Self {
                Self {
                    ptr: crate::objective_c_runtime::Id::from_ptr(ptr),
                    $($p: std::marker::PhantomData),*
                }
            }
        }
    },
}
