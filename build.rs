fn main() {
    let target = std::env::var("TARGET").unwrap();

    if target.contains("apple-ios") {
        #[cfg(feature = "uikit")]
        println!("cargo:rustc-link-lib=framework=UIKit");
    } else if target.contains("apple-darwin") {
        #[cfg(feature = "appkit")]
        println!("cargo:rustc-link-lib=framework=AppKit");
    }

    #[cfg(feature = "user_notifications")]
    println!("cargo:rustc-link-lib=framework=UserNotifications");

    #[cfg(feature = "foundation")]
    println!("cargo:rustc-link-lib=framework=Foundation");

    #[cfg(feature = "compression")]
    println!("cargo:rustc-link-lib=dylib=Compression");

    #[cfg(feature = "natural_language")]
    println!("cargo:rustc-link-lib=framework=NaturalLanguage");

    #[cfg(feature = "background_tasks")]
    println!("cargo:rustc-link-lib=framework=BackgroundTasks");

    #[cfg(feature = "core_graphics")]
    println!("cargo:rustc-link-lib=framework=CoreGraphics");

    #[cfg(feature = "core_foundation")]
    println!("cargo:rustc-link-lib=framework=CoreFoundation");

    #[cfg(feature = "contacts")]
    println!("cargo:rustc-link-lib=framework=Contacts");

    #[cfg(feature = "core_ml")]
    println!("cargo:rustc-link-lib=framework=CoreML");
}
