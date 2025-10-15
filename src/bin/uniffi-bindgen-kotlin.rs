use std::env;
use std::process;
use uniffi_bindgen::bindings::KotlinBindingGenerator;
use uniffi_bindgen::library_mode::generate_bindings;
use camino::Utf8PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("UniFFI Kotlin Binding Generator");
        eprintln!();
        eprintln!("Usage: {} <library_path> <output_dir>", args[0]);
        eprintln!();
        eprintln!("Example:");
        eprintln!("  {} target/release/libdojo_c.dylib bindings/kotlin", args[0]);
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

