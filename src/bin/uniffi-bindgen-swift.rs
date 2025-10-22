use std::env;
use std::process;
use uniffi_bindgen::bindings::{generate_swift_bindings, SwiftBindingsOptions};
use camino::Utf8PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("UniFFI Swift Binding Generator");
        eprintln!();
        eprintln!("Usage: {} <library_path> <output_dir> [--swift-sources] [--headers] [--modulemap]", args[0]);
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --swift-sources       Generate .swift source files (default)");
        eprintln!("  --headers             Generate .h header files");
        eprintln!("  --modulemap           Generate modulemap");
        eprintln!("  --xcframework         Generate XCFramework-compatible modulemap");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {} target/release/libdojo_c.dylib bindings/swift --swift-sources", args[0]);
        eprintln!("  {} target/release/libdojo_c.dylib bindings/swift --headers --modulemap", args[0]);
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
    
    // Parse options
    let has_swift_sources = args.contains(&"--swift-sources".to_string());
    let has_headers = args.contains(&"--headers".to_string());
    let has_modulemap = args.contains(&"--modulemap".to_string());
    let has_xcframework = args.contains(&"--xcframework".to_string());
    
    // Default to generating Swift sources if no specific flags are provided
    let generate_swift_sources = has_swift_sources || (!has_headers && !has_modulemap);
    
    println!("Generating Swift bindings...");
    println!("Library: {}", library_path);
    println!("Output:  {}", out_dir);
    
    let options = SwiftBindingsOptions {
        generate_swift_sources,
        generate_headers: has_headers,
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
            eprintln!("Error generating bindings: {}", e);
            process::exit(1);
        }
    }
}
