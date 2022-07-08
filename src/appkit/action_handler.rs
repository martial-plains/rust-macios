#![allow(trivial_casts, clippy::let_unit_value)]

use std::{fmt, sync::Once};

use crate::{
    foundation::NSString,
    objective_c_runtime::{
        class,
        declare::ClassDecl,
        id, msg_send,
        runtime::{Class, Object, Sel},
        sel, sel_impl,
        traits::FromId,
        ShareId,
    },
    utils::get_variable,
};

/// The pointer for `ActionHandler`
pub static ACTION_CALLBACK_PTR: &str = "RUST_Action_Callback_Ptr";

/// The action handler
pub struct Action(Box<dyn Fn() + Send + Sync + 'static>);

/// A handler that contains the class for callback storage and invocation on
pub struct ActionHandler {
    /// The class for the callback.
    pub invoker: ShareId<Object>,
    /// The callback.
    pub action: Box<Action>,
}

impl ActionHandler {
    /// Returns a new TargetEventHandler.
    pub fn new<F: Fn() + Send + Sync + 'static>(control: &Object, action: F) -> Self {
        let block = Box::new(Action(Box::new(action)));
        let ptr = Box::into_raw(block);

        let invoker = unsafe {
            ShareId::from_ptr({
                let invoker: id = msg_send![Self::register_handler_class::<F>(), alloc];
                let invoker: id = msg_send![invoker, init];
                (*invoker).set_ivar(ACTION_CALLBACK_PTR, ptr as usize);
                let _: () = msg_send![control, setAction: sel!(perform:)];
                let _: () = msg_send![control, setTarget: invoker];
                invoker
            })
        };

        ActionHandler {
            invoker,
            action: unsafe { Box::from_raw(ptr) },
        }
    }

    fn register_handler_class<F>() -> *const Class
    where
        F: Fn() + 'static,
    {
        static mut CLASS: *const Class = 0 as *const Class;
        static INIT: Once = Once::new();

        INIT.call_once(|| unsafe {
            let mut decl = ClassDecl::new("RUST_ActionHandler", class!(NSObject)).unwrap();

            decl.add_ivar::<usize>(ACTION_CALLBACK_PTR);
            decl.add_method(
                sel!(perform:),
                perform::<F> as extern "C" fn(&mut Object, _, id),
            );

            CLASS = decl.register();
        });

        unsafe { CLASS }
    }
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action")
    }
}

impl fmt::Debug for ActionHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", unsafe {
            NSString::from_id(msg_send![self.invoker, debugDescription])
        })
    }
}

/// This will fire for an NSButton callback.
extern "C" fn perform<F: Fn() + 'static>(this: &mut Object, _: Sel, _sender: id) {
    let action = get_variable::<Action>(this, ACTION_CALLBACK_PTR);
    (action.0)();
}
