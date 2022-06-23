use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

/// Embeds a manifest in the target executable with the specified contents,
/// as a resource of type RT_MANIFEST.
pub fn embed_manifest(contents: &str) {
    let mut path = PathBuf::from(std::env::var("OUT_DIR").expect("No `OUT_DIR` env variable set"));

    path.push("app.manifest");
    if let Ok(mut file) = OpenOptions::new().create_new(true).write(true).open(&path) {
        file.write_all(contents.as_bytes()).unwrap();
    }
    path.pop();

    if std::env::var("TARGET")
        .unwrap()
        .ends_with("-pc-windows-gnu")
    {
        path.push("app.rc");
        if let Ok(mut file) = OpenOptions::new().create_new(true).write(true).open(&path) {
            file.write_all(r#"1 24 app.manifest"#.as_bytes()).unwrap();
        }
        let input = &path.display().to_string();
        path.pop();

        path.push("lib__manifest.a");
        let output = &path.display().to_string();
        path.pop();

        Command::new("windres")
            .args(["-i", &input, "-o", &output])
            .spawn()
            .expect("Failed to spawn windres");

        println!("cargo:rustc-link-search={}", &path.display());
        println!("cargo:rustc-link-lib=static=__manifest");
    } else {
        path.push("app.manifest");
        println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg=/MANIFESTINPUT:{}", &path.display());
    }
}
