mod generated;

use embed_manifest::{embed_manifest, new_manifest};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

/// Writes the Windows App Runtime Bootstrap dll to `OUT_DIR` and embeds a manifest into the executable.
/// This is only useful when called from a build script.
pub fn to_output_dir() {
    let mut path =
        std::path::PathBuf::from(std::env::var("OUT_DIR").expect("No `OUT_DIR` env variable set"));
    path.pop();
    path.pop();
    path.pop();
    path.push("Microsoft.WindowsAppRuntime.Bootstrap.dll");
    if let Ok(mut file) = OpenOptions::new().create_new(true).write(true).open(path) {
        file.write_all(&generated::BOOTSTRAP_DLL_BYTES).unwrap();
    }

    embed_manifest(new_manifest(
        &env::var("CARGO_PKG_NAME").expect("Unable to read package name"),
    ))
    .expect("unable to embed manifest file");
}
