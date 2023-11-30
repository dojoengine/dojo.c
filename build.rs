use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let config = cbindgen::Config {
        language: cbindgen::Language::C,
        ..Default::default()
    };

    cbindgen::Builder::new()
        .with_config(config)
        .with_crate(crate_dir)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("dojo.h");

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
