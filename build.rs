fn main() {
    let target = std::env::var("TARGET").unwrap();

    if target.contains("-ios") {
        println!("cargo:rustc-link-lib=framework=UIKit");
    } else {
        println!("cargo:rustc-link-lib=framework=AppKit");
    }

    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=dylib=Compression");
    println!("cargo:rustc-link-lib=framework=NaturalLanguage");
    println!("cargo:rustc-link-lib=framework=BackgroundTasks");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
}
