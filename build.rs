fn main() {
    #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "watchos"))]
    #[cfg(feature = "uikit")]
    println!("cargo:rustc-link-lib=framework=UIKit");

    #[cfg(target_os = "macos")]
    #[cfg(feature = "appkit")]
    println!("cargo:rustc-link-lib=framework=AppKit");

    #[cfg(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    #[cfg(feature = "user_notifications")]
    println!("cargo:rustc-link-lib=framework=UserNotifications");

    #[cfg(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    #[cfg(feature = "foundation")]
    println!("cargo:rustc-link-lib=framework=Foundation");

    #[cfg(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    #[cfg(feature = "compression")]
    println!("cargo:rustc-link-lib=dylib=Compression");

    #[cfg(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    #[cfg(feature = "natural_language")]
    println!("cargo:rustc-link-lib=framework=NaturalLanguage");

    #[cfg(any(target_os = "ios", target_os = "tvos"))]
    #[cfg(feature = "background_tasks")]
    println!("cargo:rustc-link-lib=framework=BackgroundTasks");

    #[cfg(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    #[cfg(feature = "core_graphics")]
    println!("cargo:rustc-link-lib=framework=CoreGraphics");

    #[cfg(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    #[cfg(feature = "core_foundation")]
    println!("cargo:rustc-link-lib=framework=CoreFoundation");

    #[cfg(any(target_os = "ios", target_os = "macos", target_os = "watchos"))]
    #[cfg(feature = "contacts")]
    println!("cargo:rustc-link-lib=framework=Contacts");

    #[cfg(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    #[cfg(feature = "core_ml")]
    println!("cargo:rustc-link-lib=framework=CoreML");

    #[cfg(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    #[cfg(feature = "core_location")]
    println!("cargo:rustc-link-lib=framework=CoreLocation");
}
