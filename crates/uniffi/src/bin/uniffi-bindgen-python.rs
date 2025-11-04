use std::{env, process};

use camino::Utf8PathBuf;
use uniffi_bindgen::bindings::python::run_pipeline;
use uniffi_bindgen::cargo_metadata::CrateConfigSupplier;
use uniffi_bindgen::pipeline::initial::Root;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Show help if requested
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        eprintln!("UniFFI Python Binding Generator");
        eprintln!();
        eprintln!("Usage: {} [library_path] [output_dir]", args[0]);
        eprintln!();
        eprintln!("Arguments:");
        eprintln!(
            "  library_path          Path to the compiled library (default: \
             target/release/libdojo_uniffi.dylib)"
        );
        eprintln!(
            "  output_dir            Output directory for bindings (default: bindings/python)"
        );
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {}                    # Use defaults", args[0]);
        eprintln!("  {} target/release/libdojo_uniffi.dylib bindings/python", args[0]);
        eprintln!();
        process::exit(0);
    }

    // Determine library extension based on platform
    let lib_ext = if cfg!(target_os = "macos") {
        "dylib"
    } else if cfg!(target_os = "windows") {
        "dll"
    } else {
        "so"
    };

    // Default paths (must match the library output name)
    let default_lib = format!("target/release/libdojo_uniffi.{lib_ext}");
    let default_out = "bindings/python";

    // Parse arguments
    let positional_args: Vec<&String> =
        args.iter().skip(1).filter(|arg| !arg.starts_with("--")).collect();

    let library_path =
        Utf8PathBuf::from(positional_args.first().map(|s| s.as_str()).unwrap_or(&default_lib));
    let out_dir =
        Utf8PathBuf::from(positional_args.get(1).map(|s| s.as_str()).unwrap_or(default_out));

    if !library_path.exists() {
        eprintln!("Error: Library file not found: {library_path}");
        eprintln!("Build the library first with: cargo build --release -p dojo-uniffi");
        eprintln!();
        eprintln!("Hint: Run with --help to see usage information");
        process::exit(1);
    }

    // Create output directory if it doesn't exist
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("Error: Failed to create output directory {out_dir}: {e}");
        process::exit(1);
    }

    println!("Generating Python bindings...");
    println!("Library: {library_path}");
    println!("Output:  {out_dir}");

    // Find uniffi.toml config file
    let config_file = Utf8PathBuf::from("crates/uniffi/uniffi.toml");
    if !config_file.exists() {
        eprintln!("Warning: uniffi.toml not found at {config_file}");
    }

    // Use cargo metadata to get crate configuration
    let metadata = match cargo_metadata::MetadataCommand::new().exec() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error getting cargo metadata: {e}");
            eprintln!("Make sure you're running this from a cargo project directory");
            process::exit(1);
        }
    };

    let config_supplier = CrateConfigSupplier::from(metadata);

    match Root::from_library(config_supplier, &library_path, Some(config_file.to_string())) {
        Ok(root) => match run_pipeline(root, &out_dir) {
            Ok(_) => {
                println!("✓ Python bindings generated successfully in {out_dir}");
            }
            Err(e) => {
                eprintln!("Error generating Python bindings: {e}");
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error loading library metadata: {e}");
            process::exit(1);
        }
    }
}
