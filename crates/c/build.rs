use std::env;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Create bindings/c directory if it doesn't exist
    let bindings_dir = Path::new(&crate_dir).join("../../bindings/c");
    std::fs::create_dir_all(&bindings_dir).expect("Unable to create bindings/c directory");

    // Generate C bindings only
    cbindgen::Builder::new()
        .with_config({
            let mut config = cbindgen::Config {
                language: cbindgen::Language::C,
                braces: cbindgen::Braces::SameLine,
                cpp_compat: true,
                style: cbindgen::Style::Both,
                layout: cbindgen::LayoutConfig { ..Default::default() },
                enumeration: cbindgen::EnumConfig {
                    derive_helper_methods: true,
                    // prefix_with_name: true,
                    ..Default::default()
                },
                export: cbindgen::ExportConfig {
                    mangle: cbindgen::MangleConfig {
                        remove_underscores: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                namespace: Some("dojo_bindings".to_string()),
                ..Default::default()
            };

            config.defines.insert(
                "target_pointer_width = 32".to_string(),
                "TARGET_POINTER_WIDTH_32".to_string(),
            );

            config
        })
        .with_crate(crate_dir.clone())
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file(bindings_dir.join("dojo.h"));
}
