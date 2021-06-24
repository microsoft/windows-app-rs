use std::{env, fs, path};

fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    println!("cargo:rustc-link-search=native=.windows/lib/{}", arch);

    let source = format!(
        ".windows/lib/{}/Microsoft.ProjectReunion.Bootstrap.dll",
        arch
    );
    let target = path::Path::new(env::var("OUT_DIR").unwrap().as_str())
        .join("../../../Microsoft.ProjectReunion.Bootstrap.dll");

    fs::copy(&source, &target).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.windows");
}
