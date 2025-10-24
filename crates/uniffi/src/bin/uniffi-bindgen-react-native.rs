use std::{env, process};

use camino::Utf8PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Show help if requested
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        eprintln!("UniFFI React Native Binding Generator");
        eprintln!();
        eprintln!("Usage: {} [library_path] [output_dir] [OPTIONS]", args[0]);
        eprintln!();
        eprintln!("Arguments:");
        eprintln!(
            "  library_path          Path to the compiled library (default: \
             target/release/libdojo_uniffi.dylib)"
        );
        eprintln!(
            "  output_dir            Output directory for bindings (default: bindings/react-native)"
        );
        eprintln!();
        eprintln!("Requirements:");
        eprintln!("  This tool requires uniffi-bindgen-react-native to be installed:");
        eprintln!("  cargo install --git https://github.com/jhugman/uniffi-bindgen-react-native --branch update-uniffi-0.30 uniffi-bindgen-react-native");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {}                    # Use defaults", args[0]);
        eprintln!("  {} target/release/libdojo_uniffi.dylib bindings/react-native", args[0]);
        eprintln!();
        process::exit(0);
    }

    // Check if uniffi-bindgen-react-native is installed
    if which::which("uniffi-bindgen-react-native").is_err() {
        eprintln!("Error: uniffi-bindgen-react-native is not installed or not in PATH");
        eprintln!();
        eprintln!("Please install it with:");
        eprintln!("  cargo install --git https://github.com/jhugman/uniffi-bindgen-react-native --branch update-uniffi-0.30 uniffi-bindgen-react-native");
        eprintln!();
        eprintln!("Or add it to your PATH if already installed");
        process::exit(1);
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
    let default_lib = format!("target/release/libdojo_uniffi.{}", lib_ext);
    let default_out = "bindings/react-native";

    // Parse arguments
    let positional_args: Vec<&String> =
        args.iter().skip(1).filter(|arg| !arg.starts_with("--")).collect();

    let library_path =
        Utf8PathBuf::from(positional_args.first().map(|s| s.as_str()).unwrap_or(&default_lib));
    let out_dir =
        Utf8PathBuf::from(positional_args.get(1).map(|s| s.as_str()).unwrap_or(default_out));

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

    println!("Generating React Native bindings...");
    println!("Library: {}", library_path);
    println!("Output:  {}", out_dir);

    // Build command for uniffi-bindgen-react-native
    let mut cmd = process::Command::new("uniffi-bindgen-react-native");
    cmd.arg("--library").arg(&library_path);
    cmd.arg("--out-dir").arg(&out_dir);

    // Execute the command
    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                println!("✓ React Native bindings generated successfully!");
                
                if !output.stdout.is_empty() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
            } else {
                eprintln!("Error generating bindings:");
                if !output.stderr.is_empty() {
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }
                if !output.stdout.is_empty() {
                    eprintln!("{}", String::from_utf8_lossy(&output.stdout));
                }
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error executing uniffi-bindgen-react-native: {}", e);
            process::exit(1);
        }
    }
}

