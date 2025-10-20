use std::env;
use std::process;
use uniffi_bindgen::bindings::KotlinBindingGenerator;
use uniffi_bindgen::library_mode::generate_bindings;
use camino::Utf8PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Show help if requested
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        eprintln!("UniFFI Kotlin Binding Generator");
        eprintln!();
        eprintln!("Usage: {} [library_path] [output_dir]", args[0]);
        eprintln!();
        eprintln!("Arguments:");
        eprintln!("  library_path          Path to the compiled library (default: target/release/libdojo.dylib)");
        eprintln!("  output_dir            Output directory for bindings (default: bindings/kotlin)");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {}                    # Use defaults", args[0]);
        eprintln!("  {} target/release/libdojo.dylib bindings/kotlin", args[0]);
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
    
    // Default paths
    let default_lib = format!("target/release/libdojo.{}", lib_ext);
    let default_out = "bindings/kotlin";
    
    // Parse arguments
    let positional_args: Vec<&String> = args.iter()
        .skip(1)
        .filter(|arg| !arg.starts_with("--"))
        .collect();
    
    let library_path = Utf8PathBuf::from(
        positional_args.get(0).map(|s| s.as_str()).unwrap_or(&default_lib)
    );
    let out_dir = Utf8PathBuf::from(
        positional_args.get(1).map(|s| s.as_str()).unwrap_or(default_out)
    );
    
    if !library_path.exists() {
        eprintln!("Error: Library file not found: {}", library_path);
        eprintln!("Build the library first with: cargo build --release -p dojo-uniffi");
        eprintln!();
        eprintln!("Hint: Run with --help to see usage information");
        process::exit(1);
    }
    
    // Create output directory if it doesn't exist
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("Error: Failed to create output directory {}: {}", out_dir, e);
        process::exit(1);
    }
    
    println!("Generating Kotlin bindings...");
    println!("Library: {}", library_path);
    println!("Output:  {}", out_dir);
    
    // Use library mode with Kotlin binding generator
    let config_supplier = uniffi_bindgen::EmptyCrateConfigSupplier;
    
    match generate_bindings(
        &library_path,
        None, // crate_name (auto-detect)
        &KotlinBindingGenerator,
        &config_supplier,
        None, // config_file_override
        &out_dir,
        false, // try_format_code
    ) {
        Ok(_) => {
            println!("✓ Kotlin bindings generated successfully in {}", out_dir);
        }
        Err(e) => {
            eprintln!("Error generating bindings: {}", e);
            process::exit(1);
        }
    }
}

