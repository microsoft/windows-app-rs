fn main() {
    ::windows_app::bootstrap::deploy::to_output_dir();

    // Temporary workaround for https://github.com/microsoft/WindowsAppSDK/issues/2634
    ::windows_app::build::embed_manifest(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
         <assembly manifestVersion="1.0" xmlns="urn:schemas-microsoft-com:asm.v1" />"#,
    );

    // All Windows App SDK DLLs with non-COM/WinRT exports must be delay-loaded to give
    // developers a chance to hook LoadLibrary with the bootstrap and get the
    // Windows App Runtime package into the dll search path.

    // GNU targets already utilize delay-loaded import libraries and do not need the
    // additional linker instructions.

    if !std::env::var("TARGET").unwrap().contains("pc-windows-gnu") {
        println!("cargo:rustc-link-arg=/DELAYLOAD:dwritecore.dll");
        println!("cargo:rustc-link-lib=static=delayimp");
    }
}
