use app_kit_proc_macros::ns_application_main;
use rust_macios::appkit::{
    traits::{PNSApplicationDelegate, INSApplication}, NSApplication, NSApplicationActivationPolicy,
};

#[derive(Default, Clone)]
struct AppDelegate {}

unsafe impl Sync for AppDelegate {}

unsafe impl Send for AppDelegate {}

impl PNSApplicationDelegate for AppDelegate {
    fn did_finish_launching(&mut self) {
        println!("Done Launching");
    }
}

#[ns_application_main]
fn main() {
    let app = NSApplication::new("com.rust.macos.appkit.example", AppDelegate::default());
    app.im_set_activation_policy(NSApplicationActivationPolicy::Regular);
}
