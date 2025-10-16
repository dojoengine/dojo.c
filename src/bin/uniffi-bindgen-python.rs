use std::env;
use std::process;
use uniffi_bindgen::bindings::python::run_pipeline;
use uniffi_bindgen::cargo_metadata::CrateConfigSupplier;
use uniffi_bindgen::pipeline::initial::Root;
use camino::Utf8PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("UniFFI Python Binding Generator");
        eprintln!();
        eprintln!("Usage: {} <library_path> <output_dir>", args[0]);
        eprintln!();
        eprintln!("Example:");
        eprintln!("  {} target/release/libdojo_c.dylib bindings/python", args[0]);
        eprintln!();
        process::exit(1);
    }
    
    let library_path = Utf8PathBuf::from(&args[1]);
    let out_dir = Utf8PathBuf::from(&args[2]);
    
    if !library_path.exists() {
        eprintln!("Error: Library file not found: {}", library_path);
        eprintln!("Build the library first with: cargo build --release");
        process::exit(1);
    }
    
    println!("Generating Python bindings...");
    println!("Library: {}", library_path);
    println!("Output:  {}", out_dir);
    
    // Use cargo metadata to get crate configuration
    let metadata = match cargo_metadata::MetadataCommand::new().exec() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error getting cargo metadata: {}", e);
            eprintln!("Make sure you're running this from a cargo project directory");
            process::exit(1);
        }
    };
    
    let config_supplier = CrateConfigSupplier::from(metadata);
    
    match Root::from_library(config_supplier, &library_path, None) {
        Ok(root) => {
            match run_pipeline(root, &out_dir) {
                Ok(_) => {
                    println!("✓ Python bindings generated successfully in {}", out_dir);
                }
                Err(e) => {
                    eprintln!("Error generating Python bindings: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error loading library metadata: {}", e);
            process::exit(1);
        }
    }
}

