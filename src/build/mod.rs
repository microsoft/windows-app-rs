use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

/// Embeds a manifest in the target executable with the specified contents,
/// as a resource of type RT_MANIFEST.
pub fn embed_manifest(contents: &str) {
    let mut path = PathBuf::from(std::env::var("OUT_DIR").expect("No `OUT_DIR` env variable set"));

    path.push("app.manifest");
    if let Ok(mut file) = OpenOptions::new().create_new(true).write(true).open(&path) {
        file.write_all(contents.as_bytes()).unwrap();
    }

    println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
    println!("cargo:rustc-link-arg=/MANIFESTINPUT:{}", &path.display());
}
