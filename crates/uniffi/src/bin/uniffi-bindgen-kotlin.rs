use std::env;
use std::process;
use std::fs;
use uniffi_bindgen::{BindgenLoader, BindingGenerator};
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
        eprintln!("  library_path          Path to the compiled library (default: target/release/libdojo_uniffi.dylib)");
        eprintln!("  output_dir            Output directory for bindings (default: bindings/kotlin)");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {}                    # Use defaults", args[0]);
        eprintln!("  {} target/release/libdojo_uniffi.dylib bindings/kotlin", args[0]);
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
    let default_lib = format!("target/release/libdojo_uniffi.{}", lib_ext);
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
    if let Err(e) = fs::create_dir_all(&out_dir) {
        eprintln!("Error: Failed to create output directory {}: {}", out_dir, e);
        process::exit(1);
    }
    
    println!("Generating Kotlin bindings...");
    println!("Library: {}", library_path);
    println!("Output:  {}", out_dir);
    
    match generate_kotlin_bindings(&library_path, &out_dir) {
        Ok(_) => {
            println!("✓ Kotlin bindings generated successfully!");
        }
        Err(e) => {
            eprintln!("Error generating bindings: {}", e);
            process::exit(1);
        }
    }
}

fn generate_kotlin_bindings(library_path: &Utf8PathBuf, out_dir: &Utf8PathBuf) -> anyhow::Result<()> {
    use uniffi_bindgen::bindings::KotlinBindingGenerator;
    use uniffi_bindgen::cargo_metadata::CrateConfigSupplier;
    
    // Get cargo metadata for config
    let metadata = cargo_metadata::MetadataCommand::new()
        .exec()
        .map_err(|e| anyhow::anyhow!("Failed to get cargo metadata: {}", e))?;
    
    let config_supplier = CrateConfigSupplier::from(metadata);
    
    // Load the library metadata and components
    let loader = BindgenLoader::new(&config_supplier);
    let metadata = loader.load_metadata(library_path)?;
    let cis = loader.load_cis(metadata)?;
    
    // Parse config with Kotlin binding generator
    let generator = KotlinBindingGenerator;
    let components = loader.load_components(cis, |_ci, root_toml| {
        generator.new_config(&root_toml)
    })?;
    
    // Generate bindings using the Kotlin generator
    use uniffi_bindgen::GenerationSettings;
    let settings = GenerationSettings {
        out_dir: out_dir.clone(),
        try_format_code: false,
        cdylib: None,
    };
    
    generator.write_bindings(&settings, &components)?;
    
    Ok(())
}
