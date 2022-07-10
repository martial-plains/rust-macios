use app_kit_proc_macros::ns_application_main;

use rust_macios::{
    appkit::{
        traits::{
            INSResponder, INSTextField, INSView, INSViewController, INSWindow,
            PNSApplicationDelegate, INSApplication,
        },
        NSApplication, NSApplicationActivationPolicy, NSTextField, NSView, NSWindow,
    },
    foundation::{macros::ns_array, NSPoint, NSRect, NSSize, NSString},
    objective_c_runtime::{
        id, msg_send, nil, objc_impl, objc_impl_init, objc_selector_impl,
        runtime::{Class, Object},
        sel, sel_impl,
        traits::PNSObject,
        Id, ShareId,
    },
    uikit::{
        traits::{INSLayoutAnchor, INSLayoutConstraint},
        NSLayoutConstraint,
    },
};

pub struct ViewController {
    pub ptr: ShareId<Object>,
}

#[objc_impl(NSViewController)]
impl ViewController {
    #[objc_impl_init]
    fn init() -> Self {
        Self {
            ptr: unsafe { Id::from_ptr(msg_send![Self::im_class(), new]) },
        }
    }

    #[objc_selector_impl("viewDidLoad")]
    pub fn view_did_load(&self, _: &Object) {
        // 1: Create a view
        self.ip_set_view(NSView::im_init_with_frame(NSRect {
            origin: NSPoint { x: 0.0, y: 0.0 },
            size: NSSize {
                width: 300.0,
                height: 300.0,
            },
        }));

        // 2: Create a label
        let label = NSTextField::tm_label_with_string(NSString::from("Hello World!"));

        label.ip_set_translates_autoresizing_mask_to_constraints(false);
        let view = self.ip_view();

        view.ip_add_subview(label.clone());
        self.ip_set_view(view);

        NSLayoutConstraint::tm_activate_constraints(ns_array![
            label
                .ip_center_x_anchor()
                .im_constraint_equal_to_anchor(self.ip_view().ip_center_x_anchor()),
            label
                .ip_center_y_anchor()
                .im_constraint_equal_to_anchor(self.ip_view().ip_center_y_anchor())
        ])
    }
}

impl PNSObject for ViewController {
    fn im_class<'a>() -> &'a Class {
        unsafe { &*Self::register_class() }
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl INSResponder for ViewController {}

impl INSViewController for ViewController {}

#[derive(Default)]
struct AppDelegate {
    window: Option<NSWindow>,
}

impl AppDelegate {}

impl PNSApplicationDelegate for AppDelegate {
    fn did_finish_launching(&mut self) {
        let view_controller = ViewController::init();
        view_controller.view_did_load(&view_controller.ptr);

        let window = NSWindow::tm_window_with_content_view_controller(view_controller);

        self.window = Some(window);

        if let Some(window) = &self.window {
            window.set_minimum_content_size(NSSize {
                width: 200.0,
                height: 200.0,
            });
            window.set_title("Hello World");
            window.make_key_and_order_front(nil);
        }
    }

    fn should_terminate_after_last_window_closed(&mut self) -> bool {
        true
    }
}

#[ns_application_main]
fn main() {
    let mut app = NSApplication::shared_application();

    app.ip_set_delegate(AppDelegate::default());

    app.set_activation_policy(NSApplicationActivationPolicy::Regular);
}
