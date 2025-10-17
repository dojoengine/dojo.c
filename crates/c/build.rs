use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

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
        .expect("Unable to generate bindings")
        .write_to_file("dojo.h");

    cbindgen::Builder::new()
        .with_config({
            let mut config = cbindgen::Config {
                language: cbindgen::Language::Cxx,
                braces: cbindgen::Braces::SameLine,
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
        .expect("Unable to generate bindings")
        .write_to_file("dojo.hpp");

    cbindgen::Builder::new()
        .with_config({
            let mut config = cbindgen::Config {
                language: cbindgen::Language::Cython,
                braces: cbindgen::Braces::SameLine,
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
        .expect("Unable to generate bindings")
        .write_to_file("dojo.pyx");

    // NOTE: disable for now. use c2cs to generate csharp bindings
    // generics not supported
    // csbindgen::Builder::default()
    //     .input_extern_file("src/lib.rs")
    //     .input_extern_file("src/types.rs")
    //     .csharp_dll_name("libtorii_c.dylib")
    //     .csharp_namespace("Dojo")
    //     .generate_csharp_file("./Dojo.g.cs")
    //     .unwrap();
}
