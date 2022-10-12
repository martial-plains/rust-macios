pub macro declare_CFType {
    (
        $(#[$doc:meta])*
        $ty:ident
    ) => {
        $(#[$doc])*
        pub struct $ty(std::os::raw::c_void);

        impl Drop for $ty {
            fn drop(&mut self) {
                unsafe { $crate::core_foundation::CFType::release(std::ptr::addr_of!(self.0)) }
            }
        }

        impl $crate::core_foundation::CFTypeObject for $ty {
            fn get_internal_object(&self) -> &std::ffi::c_void {
                &self.0
            }
        }
    }
}
