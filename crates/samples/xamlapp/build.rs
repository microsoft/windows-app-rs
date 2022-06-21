fn main() {
    ::windows_app::bootstrap::deploy::to_output_dir();

    // Temporary workaround for https://github.com/microsoft/WindowsAppSDK/issues/2634
    ::windows_app::build::embed_manifest(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
           <assembly manifestVersion="1.0" xmlns="urn:schemas-microsoft-com:asm.v1" />"#,
    );
}
