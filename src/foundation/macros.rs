/// A macro to create new `NSArray`s.
pub macro ns_array {
    () => {
        $crate::foundation::NSArray::new()
    },
    ($($x:expr),*) => {
        $crate::foundation::NSArray::from(vec![$($x),*])
    },
}

/// A macro to create new `NSDictionary`s.
pub macro ns_dictionary {
    (@single $($x:tt)*) => (()),
    (@count $($rest:expr),*) => (<[()]>::len(&[$(ns_dictionary!(@single $rest)),*])),

    ($($key:expr => $value:expr,)+) => { ns_dictionary!($($key => $value),+) },
    ($($key:expr => $value:expr),*) => {
        {
            let capacity = ns_dictionary!(@count $($key),*);
            let mut map = $crate::foundation::NSMutableDictionary::with_capacity(capacity as u64);
            $(
                let _ = map.insert($key, $value);
            )*
            $crate::foundation::NSDictionary::from(map)
        }
    },
}

/// Respond to problem situations in your interactions with APIs, and fine-tune your app for better debugging.
pub macro ns_log {
    ($format:expr) => {
        use $crate::foundation::NSLog;
        unsafe {
            NSLog($format.into());
        }
    },
    ($format:expr, $($arg:expr),+) => {
        use $crate::foundation::NSLog;
        unsafe {
            NSLog($format.into(), $($arg),+);
        }
    },
}
