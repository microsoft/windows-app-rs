use std::collections::BTreeMap;
use std::io::prelude::*;
use windows_metadata::reader::*;

fn main() {
    let mut cmd = std::process::Command::new("where");
    cmd.arg("dlltool.exe");

    let output = cmd.output().unwrap();

    if !output.status.success() {
        println!("dlltool.exe not found");
        return;
    }

    let output = unsafe { core::str::from_utf8_unchecked(&output.stdout) };

    let platform = if output.contains("mingw64") {
        "x86_64_gnu"
    } else if output.contains("mingw32") {
        "i686_gnu"
    } else {
        println!("mingw not found");
        return;
    };

    println!("Platform: {}", platform);

    let reader = TypeReader::get();

    let mut libraries = BTreeMap::<String, BTreeMap<&'static str, usize>>::new();
    for namespace in [
        "Microsoft.WindowsAppSdk.Foundation",
        "Microsoft.DirectWriteCore",
    ] {
        let root = reader.types.get_namespace(&namespace).unwrap();
        load_functions(root, &mut libraries);
    }

    let output = std::path::PathBuf::from(format!("crates/targets/{}/lib", platform));
    let _ = std::fs::remove_dir_all(&output);
    std::fs::create_dir_all(&output).unwrap();

    for (library, functions) in &libraries {
        build_library(&output, library, functions, platform);
    }
}

fn load_functions(
    tree: &TypeTree,
    libraries: &mut BTreeMap<String, BTreeMap<&'static str, usize>>,
) {
    tree.types
        .values()
        .flat_map(|entry| entry.iter())
        .for_each(|def| load_function(def, libraries));

    tree.namespaces
        .values()
        .for_each(|tree| load_functions(tree, libraries));
}

fn load_function(def: &Type, libraries: &mut BTreeMap<String, BTreeMap<&'static str, usize>>) {
    if let Type::MethodDef(def) = def {
        let library = def
            .impl_map()
            .expect("Function")
            .scope()
            .name()
            .to_lowercase();

        let params = def.signature(&[]).size();

        libraries
            .entry(library)
            .or_default()
            .insert(def.name(), params);
    }
}

fn build_library(
    output: &std::path::Path,
    library: &str,
    functions: &BTreeMap<&'static str, usize>,
    platform: &str,
) {
    println!("{}", library);

    // Note that we don't use set_extension as it confuses PathBuf when the library name includes a period.

    let def_path = output.join(format!("{}.def", library));
    let mut def = std::fs::File::create(&def_path).unwrap();

    def.write_all(
        format!(
            r#"
LIBRARY {}.dll
EXPORTS
"#,
            library
        )
        .as_bytes(),
    )
    .unwrap();

    for (function, params) in functions {
        if platform.eq("i686_gnu") {
            def.write_all(format!("{}@{}\n", function, params).as_bytes())
                .unwrap();
        } else {
            def.write_all(format!("{}\n", function).as_bytes()).unwrap();
        }
    }

    drop(def);

    let mut cmd = std::process::Command::new("dlltool");
    cmd.current_dir(&output);

    if platform.eq("i686_gnu") {
        cmd.arg("--kill-at");
    }

    cmd.arg("--input-def");
    cmd.arg(format!("{}.def", library));
    cmd.arg("--output-delaylib");
    cmd.arg(format!("lib{}.a", library));
    cmd.output().unwrap();

    std::fs::remove_file(output.join(format!("{}.def", library))).unwrap();
}
