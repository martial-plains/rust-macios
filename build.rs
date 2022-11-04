fn main() {
    #[cfg(any(target_os = "ios", target_os = "tvos", target_os = "watchos"))]
    println!("cargo:rustc-link-lib=framework=UIKit");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=framework=AppKit");

    #[cfg(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    println!("cargo:rustc-link-lib=framework=UserNotifications");

    #[cfg(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    println!("cargo:rustc-link-lib=framework=Foundation");

    #[cfg(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    println!("cargo:rustc-link-lib=dylib=Compression");

    #[cfg(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    println!("cargo:rustc-link-lib=framework=NaturalLanguage");

    #[cfg(any(target_os = "ios", target_os = "tvos"))]
    println!("cargo:rustc-link-lib=framework=BackgroundTasks");

    #[cfg(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    println!("cargo:rustc-link-lib=framework=CoreGraphics");

    #[cfg(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    println!("cargo:rustc-link-lib=framework=CoreFoundation");

    #[cfg(any(target_os = "ios", target_os = "macos", target_os = "watchos"))]
    println!("cargo:rustc-link-lib=framework=Contacts");

    #[cfg(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    println!("cargo:rustc-link-lib=framework=CoreML");

    #[cfg(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos"
    ))]
    println!("cargo:rustc-link-lib=framework=CoreLocation");
}
