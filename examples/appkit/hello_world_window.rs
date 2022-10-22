use app_kit_proc_macros::ns_application_main;

use rust_macios::{
    appkit::{
        INSApplication, INSResponder, INSView, INSViewController, INSWindow, NSApplication,
        NSApplicationActivationPolicy, NSTextField, NSView, NSWindow, PNSApplicationDelegate,
    },
    foundation::{macros::nsarray, NSPoint, NSRect, NSSize, NSString},
    objective_c_runtime::{
        class_init, id, msg_send, nil, objc_sel, register_class, runtime::Object, sel, sel_impl,
        traits::PNSObject, Id, ShareId,
    },
    uikit::{INSLayoutAnchor, NSLayoutConstraint},
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
                width: 500.0,
                height: 300.0,
            },
        }));

        // 2: Create a label
        let label = NSTextField::label_with_string(NSString::from("Hello World!"));

        label.p_set_translates_autoresizing_mask_to_constraints(false);
        let view = self.p_view();

        view.add_subview(label.clone());
        self.p_set_view(view);

        NSLayoutConstraint::activate_constraints(nsarray![
            label
                .p_center_x_anchor()
                .m_constraint_equal_to_anchor(self.p_view().center_x_anchor()),
            label
                .p_center_y_anchor()
                .m_constraint_equal_to_anchor(self.p_view().center_y_anchor())
        ])
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
        view_controller.view_did_load();

        let window = NSWindow::tm_window_with_content_view_controller(view_controller);

        self.window = Some(window);

        if let Some(window) = &mut self.window {
            window.set_minimum_content_size(NSSize {
                width: 200.0,
                height: 200.0,
            });
            window.set_title("Hello World!");
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

    app.p_set_delegate(AppDelegate::default());
    app.set_activation_policy(NSApplicationActivationPolicy::Regular);
    app.m_activate_ignoring_other_apps(true);
}
