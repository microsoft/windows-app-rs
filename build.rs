fn main() {
    let arch = match std::env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "x86_64" => "x64",
        "x86" => "x86",
        "arm" => "arm",
        "aarch64" => "arm64",
        unexpected => panic!(
            "Unexpected `{}` architecture set by `CARGO_CFG_TARGET_ARCH`",
            unexpected
        ),
    };
    println!("cargo:rustc-link-search=native=.windows/{}", arch);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.windows");
}
