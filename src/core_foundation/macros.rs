pub macro declare_CFType {
    (
        $(#[$doc:meta])*
        $ty:ident, $raw:ident
    ) => {
        $(#[$doc])*
        pub struct $ty($raw);

        impl Drop for $ty {
            #[allow(trivial_casts)]
            fn drop(&mut self) {
                unsafe {
                    use $crate::core_foundation::CFTypeObject;
                    $crate::core_foundation::CFType::release(self.get_internal_object() as $crate::core_foundation::CFTypeRef) }
            }
        }

        impl $crate::core_foundation::CFTypeObject for $ty {
            type Ref = $raw;
            #[allow(trivial_casts)]
            fn get_internal_object(&self) -> Self::Ref {
                self.0
            }

            fn create_with_ref(obj: Self::Ref) -> Self {
                Self(obj)
            }
        }
    }
}
