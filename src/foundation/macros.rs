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

#[macro_export]
/// A macro to create new `NSDictionary`s.
macro_rules! NSDictionary {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(NSDictionary!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { NSDictionary!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let capacity = NSDictionary!(@count $($key),*);
            let mut map = $crate::foundation::NSMutableDictionary::from(capacity as $crate::foundation::UInt);
            $(
                let _ = map.set_object($key, $value);
            )*
            $crate::foundation::NSDictionary::from(map)
        }
    };
}
