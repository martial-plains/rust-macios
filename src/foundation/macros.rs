#[macro_export]
/// A macro to create new `NSArray`s.
macro_rules! NSArray {
    () => {
        $crate::foundation::NSArray::new()
    };
    ($($x:expr),*) => {
        $crate::foundation::NSArray::from(vec![$($x),*])
    };
}
