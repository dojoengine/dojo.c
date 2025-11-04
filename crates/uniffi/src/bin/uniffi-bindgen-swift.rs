use std::{env, process};

use camino::Utf8PathBuf;
use uniffi_bindgen::bindings::{generate_swift_bindings, SwiftBindingsOptions};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Show help if requested
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        eprintln!("UniFFI Swift Binding Generator");
        eprintln!();
        eprintln!("Usage: {} [library_path] [output_dir] [OPTIONS]", args[0]);
        eprintln!();
        eprintln!("Arguments:");
        eprintln!(
            "  library_path          Path to the compiled library (default: \
             target/release/libdojo_uniffi.dylib)"
        );
        eprintln!(
            "  output_dir            Output directory for bindings (default: bindings/swift)"
        );
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --swift-sources       Generate .swift source files (default)");
        eprintln!("  --headers             Generate .h header files");
        eprintln!("  --modulemap           Generate modulemap");
        eprintln!("  --xcframework         Generate XCFramework-compatible modulemap");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {}                    # Use defaults", args[0]);
        eprintln!("  {} target/release/libdojo_uniffi.dylib bindings/swift", args[0]);
        eprintln!(
            "  {} target/release/libdojo_uniffi.dylib bindings/swift --swift-sources",
            args[0]
        );
        eprintln!(
            "  {} target/release/libdojo_uniffi.dylib bindings/swift --headers --modulemap",
            args[0]
        );
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
    let default_out = "bindings/swift";

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

    // Parse options
    let has_swift_sources = args.contains(&"--swift-sources".to_string());
    let has_headers = args.contains(&"--headers".to_string());
    let has_modulemap = args.contains(&"--modulemap".to_string());
    let has_xcframework = args.contains(&"--xcframework".to_string());

    // Default to generating Swift sources and headers if no specific flags are provided
    let generate_swift_sources = has_swift_sources || has_headers || !has_modulemap;
    let generate_headers = has_headers || has_swift_sources || !has_modulemap;

    println!("Generating Swift bindings...");
    println!("Library: {library_path}");
    println!("Output:  {out_dir}");

    let options = SwiftBindingsOptions {
        generate_swift_sources,
        generate_headers,
        generate_modulemap: has_modulemap,
        source: library_path,
        out_dir,
        xcframework: has_xcframework,
        module_name: Some("DojoEngine".to_string()),
        modulemap_filename: None,
        metadata_no_deps: false,
        link_frameworks: vec![],
    };

    match generate_swift_bindings(options) {
        Ok(_) => {
            println!("✓ Swift bindings generated successfully!");
        }
        Err(e) => {
            eprintln!("Error generating bindings: {e}");
            process::exit(1);
        }
    }
}
