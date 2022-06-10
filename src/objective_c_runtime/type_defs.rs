use objc::runtime::Object;

/// An ID for an Objective-C object.
#[allow(non_camel_case_types)]
pub type id = *mut Object;
