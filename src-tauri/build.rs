use std::path::PathBuf;

use normpath::PathExt;

fn main() {
    // include path openvr/headers
    let include_path = relative("openvr/headers");
    // This assumes all your C++ bindings are in main.rs
    let mut b = autocxx_build::Builder::new(relative("src/main.rs"), [&include_path])
        .build()
        .expect("Could not autogenerate bindings");
    // arbitrary library name, pick anything
    b.flag_if_supported("-std=c++14").compile("foobar");
    // Link the C++ libraries
    #[cfg(target_os = "windows")]
    let input_files = [
        relative("openvr/bin/win64/openvr_api.dll"),
        relative("openvr/lib/win64/openvr_api.lib"),
    ];
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    for f in input_files {
        let file_name = f.file_name().unwrap();
        std::fs::copy(&f, out_dir.join(file_name)).unwrap_or_else(|err| {
            panic!("Failed to copy {:?} to {:?}: {err}", file_name, &out_dir)
        });
    }
    // Run tauri build
    tauri_build::build()
}

fn relative(s: &str) -> PathBuf {
    let result = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    result.join(s).normalize().unwrap().into_path_buf()
}
