use std::sync::{Arc, Mutex};

use rust_macios::{
    appkit::{
        ns_application_main, INSApplication, INSButton, INSLayoutAnchor, INSResponder, INSView,
        INSViewController, NSApplication, NSApplicationActivationPolicy, NSLayoutConstraint,
        NSPopover, NSPopoverBehavior, NSStatusBar, NSStatusItem, NSTextField, NSView,
        PNSApplicationDelegate,
    },
    foundation::{NSPoint, NSRect, NSRectEdge, NSSize, NSString},
    nsarray,
    objective_c_runtime::{
        class_init, id, msg_send, nil, objc_sel, register_class, runtime::Object, sel, sel_impl,
        traits::PNSObject, Id, ShareId,
    },
};

pub struct ViewController {
    pub ptr: ShareId<Object>,
}

#[register_class(NSViewController)]
impl ViewController {
    #[class_init]
    fn init() -> Self {
        Self {
            ptr: unsafe { Id::from_ptr(msg_send![Self::m_class(), new]) },
        }
    }

    #[objc_sel("viewDidLoad")]
    pub fn view_did_load(&self) {
        // 1: Create a view
        self.p_set_view(NSView::init_with_frame(NSRect {
            origin: NSPoint { x: 0.0, y: 0.0 },
            size: NSSize {
                width: 300.0,
                height: 300.0,
            },
        }));

        // 2: Create a label
        let label =
            NSTextField::label_with_string(NSString::from("NSViewController without Storyboard"));

        label.p_set_translates_autoresizing_mask_to_constraints(false);
        let view = self.p_view();

        view.p_add_subview(label.clone());
        self.p_set_view(view);

        NSLayoutConstraint::activate_constraints(nsarray![
            label
                .p_center_x_anchor()
                .m_constraint_equal_to_anchor(self.p_view().p_center_x_anchor()),
            label
                .p_center_y_anchor()
                .m_constraint_equal_to_anchor(self.p_view().p_center_y_anchor())
        ])
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
        if let Some(button) = self.status_bar_item.button() {
            if self.popover.shown() {
                self.popover.perform_close(sender)
            } else {
                self.popover.show_relative_to_rect_of_view_preferred_edge(
                    button.p_bounds(),
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
        self.popover.set_behavior(NSPopoverBehavior::Transient);

        let view_controller = ViewController::init();

        view_controller.view_did_load();
        self.popover.set_content_view_controller(view_controller);

        self.status_bar_item = NSStatusBar::system_status_bar()
            .status_item_with_length(NSStatusItem::NSVARIABLE_STATUS_ITEM_LENGTH);

        if let Some(mut button) = self.status_bar_item.button() {
            button.p_set_title("🦀".into());

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

    app.p_set_delegate(AppDelegate::default());
    app.set_activation_policy(NSApplicationActivationPolicy::Regular);
}
