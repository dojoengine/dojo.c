use std::path::Path;

use diplomat_tool::{
    config::{Config, SharedConfig},
    DocsUrlGenerator,
};

pub fn main() {
    // Generate C bindings
    diplomat_tool::gen(
        Path::new("src/lib.rs"),
        "c",
        Path::new("bindings/c"),
        &DocsUrlGenerator::default(),
        Config {
            shared_config: SharedConfig {
                lib_name: Some("dojo_c".to_string()),
                ..Default::default()
            },
            ..Config::default()
        },
        false,
    )
    .unwrap();

    // Generate C++ bindings
    diplomat_tool::gen(
        Path::new("src/lib.rs"),
        "cpp",
        Path::new("bindings/cpp"),
        &DocsUrlGenerator::default(),
        Config {
            shared_config: SharedConfig {
                lib_name: Some("dojo_c".to_string()),
                ..Default::default()
            },
            ..Config::default()
        },
        false,
    )
    .unwrap();

    // Generate JavaScript/WASM bindings
    diplomat_tool::gen(
        Path::new("src/lib.rs"),
        "js",
        Path::new("bindings/js"),
        &DocsUrlGenerator::default(),
        Config {
            shared_config: SharedConfig {
                lib_name: Some("dojo_c".to_string()),
                ..Default::default()
            },
            ..Config::default()
        },
        false,
    )
    .unwrap();
}
