use quote::*;
use rayon::prelude::*;
use std::{fs::OpenOptions, io::prelude::*};

fn main() {
    let start = std::time::Instant::now();

    let mut output = std::path::PathBuf::from(reader::workspace_dir());
    generate_bootstrap(&mut output);

    output.push("src/Microsoft");
    let _ = std::fs::remove_dir_all(&output);
    output.pop();

    let reader = reader::TypeReader::get_mut();
    include_all(&mut reader.types);

    let root = reader.types.get_namespace("Microsoft").unwrap();

    let mut trees = Vec::new();
    collect_trees(&output, root.namespace, root, &mut trees);
    trees
        .par_iter()
        .for_each(|tree| gen_tree(&output, root.namespace, tree));

    output.pop();
    output.push("Cargo.toml");
    write_toml(&output, root);

    println!("Elapsed: {} ms", start.elapsed().as_millis());
}

fn write_toml(output: &std::path::Path, tree: &reader::TypeTree) {
    let mut file = std::fs::File::create(&output).unwrap();

    file.write_all(
        r#"[package]
name = "windows-app"
version = "0.2.0"
authors = [""]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Windows App SDK"
repository = "https://github.com/microsoft/windows-app-rs"
documentation = ""
readme = ".github/readme.md"
exclude = [".github", ".windows", "docs", "tests"]

[workspace]
members = [
    "crates/tools/*",
    "crates/targets/*"
]

[target.i686-pc-windows-msvc.dependencies]
windows_app_i686_msvc = { path = "crates/targets/i686_msvc", version = "0.2.0" }

[target.x86_64-pc-windows-msvc.dependencies]
windows_app_x86_64_msvc = { path = "crates/targets/x86_64_msvc", version = "0.2.0" }

[target.aarch64-pc-windows-msvc.dependencies]
windows_app_aarch64_msvc = { path = "crates/targets/aarch64_msvc", version = "0.2.0" }

[package.metadata.docs.rs]
default-target = "x86_64-pc-windows-msvc"
targets = []

[dependencies.windows]
version = "0.28"
features = [
    "std",
    "Foundation_Collections",
    "Win32_Foundation",
    "Win32_Storage_Packaging_Appx"
]

[features]
default = []
deprecated = []
"#
        .as_bytes(),
    )
    .unwrap();

    write_features(&mut file, tree.namespace, tree);
}

fn write_features(file: &mut std::fs::File, root: &'static str, tree: &reader::TypeTree) {
    for tree in tree.namespaces.values() {
        write_feature(file, root, tree);
        write_features(file, root, tree);
    }
}

fn write_feature(file: &mut std::fs::File, root: &'static str, tree: &reader::TypeTree) {
    let feature = tree.namespace[root.len() + 1..].replace('.', "_");

    let mut features = reader::BTreeSet::new();
    let mut keys = std::collections::HashSet::new();
    tree.features(&mut features, &mut keys);

    file.write_all(format!("{} = [\n", feature).as_bytes())
        .unwrap();

    if let Some(pos) = feature.rfind('_') {
        let dependency = &feature[..pos];
        file.write_all(format!("  \"{}\",\n", dependency).as_bytes())
            .unwrap();
    }

    features
        .iter()
        .filter(|f| f.starts_with("Windows."))
        .map(|f| f[8..].replace('.', "_"))
        .for_each(|f| {
            file.write_all(format!("  \"windows/{}\",\n", f).as_bytes())
                .unwrap()
        });

    file.write_all("]\n".as_bytes()).unwrap();
}

fn include_all(tree: &mut reader::TypeTree) {
    tree.include = true;

    tree.types.values_mut().for_each(|entry| {
        entry.include = reader::TypeInclude::Full;
    });

    tree.namespaces.values_mut().for_each(include_all);
}

fn collect_trees<'a>(
    output: &std::path::Path,
    root: &'static str,
    tree: &'a reader::TypeTree,
    trees: &mut Vec<&'a reader::TypeTree>,
) {
    trees.push(tree);

    tree.namespaces
        .values()
        .for_each(|tree| collect_trees(output, root, tree, trees));

    let mut path = std::path::PathBuf::from(output);
    path.push(tree.namespace.replace('.', "/"));
    std::fs::create_dir_all(&path).unwrap();
}

fn gen_tree(output: &std::path::Path, root: &'static str, tree: &reader::TypeTree) {
    let mut path = std::path::PathBuf::from(output);

    path.push(tree.namespace.replace('.', "/"));
    path.push("mod.rs");

    let tokens = gen::gen_source_file(root, tree, true);

    let mut child = std::process::Command::new("rustfmt")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn `rustfmt`");
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(tokens.into_string().as_bytes()).unwrap();
    drop(stdin);

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    std::fs::write(
        &path,
        String::from_utf8(output.stdout).expect("Failed to parse UTF-8"),
    )
    .unwrap();
}

fn generate_bootstrap(output: &mut std::path::PathBuf) {
    let x86 = include_bytes!("../bootstrap/x86/Microsoft.WindowsAppRuntime.Bootstrap.dll");
    let x86_length = x86.len();
    let x64 = include_bytes!("../bootstrap/x64/Microsoft.WindowsAppRuntime.Bootstrap.dll");
    let x64_length = x64.len();
    let arm64 = include_bytes!("../bootstrap/arm64/Microsoft.WindowsAppRuntime.Bootstrap.dll");
    let arm64_length = arm64.len();

    let tokens = quote! {
        #[cfg(target_arch = "x86")]
        pub static BOOTSTRAP_DLL_BYTES:[u8;#x86_length] = [ #(#x86,)* ];
        #[cfg(target_arch = "x86_64")]
        pub static BOOTSTRAP_DLL_BYTES:[u8;#x64_length] = [ #(#x64,)* ];
        #[cfg(target_arch = "arm64")]
        pub static BOOTSTRAP_DLL_BYTES:[u8;#arm64_length] = [ #(#arm64,)* ];
    };

    output.push("src/bootstrap/deploy/generated.rs");
    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&output)
        .unwrap()
        .write_all(tokens.to_string().as_bytes())
        .unwrap();
    output.pop();
    output.pop();
    output.pop();
    output.pop();
}
