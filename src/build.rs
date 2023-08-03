// build.rs
// todo: set target for macOS


fn main() {
    let target = std::env::var("TARGET").unwrap();

    if target.contains("x86_64-pc-windows-gnu") {
        // Set environment variables for Windows cross-compilation
        println!("cargo:rustc-env=CC=x86_64-w64-mingw32-gcc");
        println!("cargo:rustc-env=CXX=x86_64-w64-mingw32-g++");
    } /* else if target.contains("x86_64-apple-darwin") {
        // Set environment variables for macOS cross-compilation
        println!("cargo:rustc-env=CC=o64-clang");
        println!("cargo:rustc-env=CXX=o64-clang++");
    } */
}
