use rust_macios::{
    appkit::{NSApplication, NSWindow, PNSApplicationDelegate},
    foundation::NSEdgeInsetsZero,
};

#[derive(Default)]
struct BasicApp {
    window: NSWindow,
}

impl PNSApplicationDelegate for BasicApp {
    fn did_finish_launching(&self) {
        self.window.set_minimum_content_size(400., 400.);
        self.window.set_title("Hello World!");
        self.window.show();
    }

    fn did_become_active(&self) {
        println!("did_become_active");
    }
}

fn main() {
    let mut app = NSApplication::new("com.hello.world", BasicApp::default());

    NSApplication::activate(&app);
    app.run();
}
