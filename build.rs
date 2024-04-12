use std::{collections::HashMap, env};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_config({
            let mut config = cbindgen::Config::default();

            config.language = cbindgen::Language::C;
            config.braces = cbindgen::Braces::SameLine;
            config.cpp_compat = true;
            config.style = cbindgen::Style::Both;
            config.layout = cbindgen::LayoutConfig {
                ..Default::default()
            };
            config.enumeration = cbindgen::EnumConfig {
                derive_helper_methods: true,
                // prefix_with_name: true,
                ..Default::default()
            };
            config.export = cbindgen::ExportConfig {
                mangle: cbindgen::MangleConfig {
                    remove_underscores: true,
                    ..Default::default()
                },
                ..Default::default()
            };

            config.defines = HashMap::new();
            // config.defines.insert("target_arch = wasm32".to_string(), "TARGET_WASM32".to_string());
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
            let mut config = cbindgen::Config::default();

            config.language = cbindgen::Language::Cxx;
            config.braces = cbindgen::Braces::SameLine;
            config.style = cbindgen::Style::Both;
            config.layout = cbindgen::LayoutConfig {
                ..Default::default()
            };
            config.enumeration = cbindgen::EnumConfig {
                derive_helper_methods: true,
                // prefix_with_name: true,
                ..Default::default()
            };
            config.export = cbindgen::ExportConfig {
                mangle: cbindgen::MangleConfig {
                    remove_underscores: true,
                    ..Default::default()
                },
                ..Default::default()
            };

            // config.structure.derive_constructor = true;
            // config.structure.derive_eq = true;

            config.defines = HashMap::new();
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
