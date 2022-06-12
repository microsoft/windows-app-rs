use quote::*;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::read_dir;
use std::{fs::OpenOptions, io::prelude::*};
use windows_bindgen as bindgen;
use windows_metadata as metadata;

fn main() {
    let libs = std::path::PathBuf::from("crates/libs/");
    let windows_app_deploy = libs.join("windows-app-deploy/");
    gen_bootstrap(&windows_app_deploy);

    let windows_app = libs.join("windows-app/");

    let _ = std::fs::remove_dir_all(&windows_app.join("src/Microsoft/"));

    let files = read_dir(".windows/winmd")
        .unwrap()
        .filter_map(|e| {
            e.ok()
                .filter(|e| e.file_type().unwrap().is_file())
                .map(|e| e.path().into_os_string().into_string().unwrap())
        })
        .chain([
            "../windows-rs/crates/libs/metadata/default/Windows.winmd".into(),
            "../windows-rs/crates/libs/metadata/default/Windows.Win32.winmd".into(),
            "../windows-rs/crates/libs/metadata/default/Windows.Win32.Interop.winmd".into(),
        ])
        .map(|e| metadata::reader::File::new(&e).unwrap())
        .collect::<Vec<_>>();
    let reader = &windows_metadata::reader::Reader::new(&files);
    let root = reader.tree("Microsoft", &[]).unwrap();

    let window_app_src = windows_app.join("src/");
    let trees = root.flatten();
    trees
        .par_iter()
        .for_each(|tree| gen_tree(reader, &window_app_src, tree));

    let mut file = std::fs::File::create(&windows_app.join("Cargo.toml")).unwrap();

    file.write_all(
        r#"[package]
name = "windows-app"
version = "0.3.0"
authors = [""]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Windows App SDK"
repository = "https://github.com/microsoft/windows-app-rs"
documentation = ""
readme = ".github/readme.md"

[target.i686-pc-windows-msvc.dependencies]
windows_app_i686_msvc = { path = "../../targets/i686_msvc", version = "0.3.0" }

[target.x86_64-pc-windows-msvc.dependencies]
windows_app_x86_64_msvc = { path = "../../targets/x86_64_msvc", version = "0.3.0" }

[target.aarch64-pc-windows-msvc.dependencies]
windows_app_aarch64_msvc = { path = "../../targets/aarch64_msvc", version = "0.3.0" }

[package.metadata.docs.rs]
default-target = "x86_64-pc-windows-msvc"
targets = []

[dependencies.windows]
git = "https://github.com/microsoft/windows-rs"
features = [
    "alloc",
]

[features]
default = []
bootstrap = ["WindowsAppSdk_Foundation"]
deprecated = []
implement = ["windows/implement"]
"#
        .as_bytes(),
    )
    .unwrap();

    // Skip the root Windows tree while writing features
    for tree in trees.into_iter().skip(1) {
        let dependencies = reader
            .namespace_types(tree.namespace)
            .flat_map(|t| reader.type_def_methods(t))
            .flat_map(|t| {
                let sig = reader.method_def_signature(t, &[]);
                reader
                    .signature_cfg(&sig)
                    .types
                    .keys()
                    .cloned()
                    .collect::<HashSet<_>>()
            })
            .collect::<HashSet<_>>();

        let mut sub_features = Vec::<String>::new();
        let feature = tree.namespace[root.namespace.len() + 1..].replace('.', "_");
        if let Some(pos) = feature.rfind('_') {
            sub_features.push(feature[..pos].to_string());
        }

        dependencies
            .iter()
            .filter(|f| f.starts_with("Windows."))
            .map(|f| f[8..].replace('.', "_"))
            .for_each(|f| sub_features.push(format!("windows/{}", f)));

        let features = sub_features
            .iter()
            .map(|f| format!("\"{f}\""))
            .collect::<Vec<_>>()
            .join(",");

        file.write_all(format!("{} = [{}]\n", feature, features).as_bytes())
            .unwrap();
    }
}

fn gen_tree(
    reader: &metadata::reader::Reader,
    output: &std::path::Path,
    tree: &metadata::reader::Tree,
) {
    println!("{}", tree.namespace);
    let mut path = std::path::PathBuf::from(output);
    path.push(tree.namespace.replace('.', "/"));
    std::fs::create_dir_all(&path).unwrap();

    let mut gen = windows_bindgen::Gen::new(reader);
    gen.namespace = tree.namespace;
    gen.cfg = true;
    gen.doc = true;
    gen.min_xaml = true;
    gen.windows_extern = true;
    let mut tokens = bindgen::namespace(&gen, tree);
    tokens.push_str(r#"#[cfg(feature = "implement")] ::core::include!("impl.rs");"#);
    fmt_tokens(tree.namespace, &mut tokens);
    std::fs::write(path.join("mod.rs"), tokens).unwrap();

    let mut tokens = bindgen::namespace_impl(&gen, tree);
    fmt_tokens(tree.namespace, &mut tokens);
    std::fs::write(path.join("impl.rs"), tokens).unwrap();
}

fn gen_bootstrap(output: &std::path::Path) {
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

    let output = output.join("src/generated.rs");
    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&output)
        .unwrap()
        .write_all(tokens.to_string().as_bytes())
        .unwrap();
}

fn fmt_tokens(namespace: &str, tokens: &mut String) {
    let mut child = std::process::Command::new("rustfmt")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to spawn `rustfmt`");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(tokens.as_bytes()).unwrap();
    drop(stdin);
    let output = child.wait_with_output().unwrap();

    if output.status.success() {
        *tokens = String::from_utf8(output.stdout).expect("Failed to parse UTF-8");
    } else {
        println!("** {} - rustfmt failed", namespace);
    }
}
