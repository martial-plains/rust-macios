use super::{object, traits::INSRunningApplication, NSApplicationActivationOptions};

object! {
    /// An object that can manipulate and provide information for a single instance of an app.
    unsafe pub struct NSRunningApplication;
}

impl NSRunningApplication {
    /// Returns the application instance, creating it if it doesn’t exist yet.
    pub fn current_application() -> Self {
        Self::tp_current_application()
    }

    /// Indicates whether the application is currently frontmost.
    pub fn active(&self) -> bool {
        self.ip_is_active()
    }

    /// Attempts to activate the application using the specified options.
    pub fn activate_with_options(&mut self, options: NSApplicationActivationOptions) {
        self.im_activate_with_options(options)
    }
}

impl INSRunningApplication for NSRunningApplication {}
