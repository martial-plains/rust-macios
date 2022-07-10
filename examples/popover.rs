use std::sync::{Arc, Mutex};

use rust_macios::{
    appkit::{
        ns_application_main,
        traits::{
            INSButton, INSPopover, INSResponder, INSStatusBar, INSStatusItem, INSTextField,
            INSView, INSViewController, PNSApplicationDelegate, INSApplication,
        },
        NSApplication, NSApplicationActivationPolicy, NSPopover, NSPopoverBehavior, NSStatusBar,
        NSStatusItem, NSTextField, NSView,
    },
    foundation::{macros::ns_array, NSPoint, NSRect, NSRectEdge, NSSize, NSString},
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
        let label = NSTextField::tm_label_with_string(NSString::from(
            "NSViewController without Storyboard",
        ));

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

#[derive(Default, Clone)]
struct AppDelegate {
    status_bar_item: NSStatusItem,
    popover: NSPopover,
}

impl AppDelegate {
    pub fn toggle_popover(&self, sender: id) {
        if let Some(button) = self.status_bar_item.ip_button() {
            if self.popover.ip_shown() {
                self.popover.im_perform_close(sender)
            } else {
                self.popover
                    .im_show_relative_to_rect_of_view_preferred_edge(
                        button.ip_bounds(),
                        button,
                        NSRectEdge::MinY,
                    )
            }
        }
    }
}

unsafe impl Sync for AppDelegate {}

unsafe impl Send for AppDelegate {}

impl PNSApplicationDelegate for AppDelegate {
    fn did_finish_launching(&mut self) {
        // Create a popover
        self.popover.ip_set_behavior(NSPopoverBehavior::Transient);

        let view_controller = ViewController::init();

        view_controller.view_did_load(&view_controller.ptr);
        self.popover.ip_set_content_view_controller(view_controller);

        self.status_bar_item = NSStatusBar::tp_system_status_bar()
            .im_status_item_with_length(NSStatusItem::NSVARIABLE_STATUS_ITEM_LENGTH);

        if let Some(mut button) = self.status_bar_item.ip_button() {
            button.ip_set_title("🦀".into());

            let mutex = Arc::new(Mutex::new(self.clone()));

            let _mutex_cloned = mutex.clone();

            let tmp_callback = Box::new(move || {
                let mutex = mutex.lock().unwrap();
                mutex.toggle_popover(nil);
            });

            button.set_action(tmp_callback);
        }
    }
}

#[ns_application_main]
fn main() {
    let mut app = NSApplication::shared_application();

    app.ip_set_delegate(AppDelegate::default());
    app.set_activation_policy(NSApplicationActivationPolicy::Regular);
}
