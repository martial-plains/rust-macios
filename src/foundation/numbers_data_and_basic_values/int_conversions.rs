use objc::{class, msg_send, sel, sel_impl};

use crate::{foundation::String, id, objective_c_runtime::NSObject};

use super::Int;

impl NSObject for Int {
    fn init() -> Self {
        todo!()
    }

    fn to_id(self) -> id {
        unsafe {
            let cls = class!(NSNumber);
            let result: id = msg_send![cls, numberWithInt: self];
            let _: () = msg_send![cls, release];
            result
        }
    }

    fn from_id(_obj: id) -> Self {
        todo!()
    }

    fn description(&self) -> String {
        unsafe {
            let cls = class!(NSNumber);
            let obj: id = msg_send![cls, initWith: self];
            let result: String = msg_send![obj, description];
            let _: () = msg_send![cls, release];
            result
        }
    }

    fn debug_description(&self) -> String {
        todo!()
    }
}
