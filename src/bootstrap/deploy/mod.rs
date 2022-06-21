mod generated;

use std::fs::OpenOptions;
use std::io::Write;

/// Writes the Windows App Runtime Bootstrap dll to `OUT_DIR`.
/// This is only useful when called from a build script.
pub fn to_output_dir() {
    let mut path =
        std::path::PathBuf::from(std::env::var("OUT_DIR").expect("No `OUT_DIR` env variable set"));
    path.pop();
    path.pop();
    path.pop();
    path.push("Microsoft.WindowsAppRuntime.Bootstrap.dll");

    let bytes: &[u8] = match std::env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "x86" => &generated::BOOTSTRAP_DLL_BYTES_X86,
        "x86_64" => &generated::BOOTSTRAP_DLL_BYTES_X86_64,
        "aarch64" => &generated::BOOTSTRAP_DLL_BYTES_AARCH64,
        _ => panic!("Unsupported target architecture"),
    };

    if let Ok(mut file) = OpenOptions::new().create_new(true).write(true).open(&path) {
        file.write_all(bytes).unwrap();
    }
    println!("cargo:rerun-if-changed={}", path.display());
}
