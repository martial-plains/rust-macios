use rust_macios::appkit::{
    ns_application_main, INSApplication, NSApplication, NSApplicationActivationPolicy,
    PNSApplicationDelegate,
};

#[derive(Default, Clone)]
struct AppDelegate {
    name: String,
}

unsafe impl Sync for AppDelegate {}

unsafe impl Send for AppDelegate {}

impl PNSApplicationDelegate for AppDelegate {
    fn did_finish_launching(&mut self) {
        self.name = "Hello".to_string();
        println!("Done Launching. \n{}", self.name);
    }
}

#[ns_application_main]
fn main() {
    let mut app = NSApplication::shared_application();

    app.p_set_delegate(AppDelegate::default());
    app.set_activation_policy(NSApplicationActivationPolicy::Regular);
}
