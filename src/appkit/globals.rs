use libc::{c_char, c_int};

extern "C" {
    /// Called by the main function to create and run the application.
    ///
    /// # Arguments
    ///
    /// * `argc` - The number of arguments passed to the program.
    /// * `argv` - The arguments passed to the program.
    ///
    /// # Returns
    ///
    /// This method never returns a result code. Instead, it calls the exit
    /// function to exit the application and terminate the process. If you want
    /// to determine why the application exited, you should look at the result
    /// code from the exit function instead.
    pub fn NSApplicationMain(argc: c_int, argv: *const *const c_char) -> c_int;
}
