use std::{env, process};

use camino::Utf8PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Show help if requested
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        eprintln!("UniFFI C++ Binding Generator for Dojo");
        eprintln!();
        eprintln!("Usage: {} [udl_path] [output_dir] [OPTIONS]", args[0]);
        eprintln!();
        eprintln!("Arguments:");
        eprintln!("  udl_path              Path to the .udl file (default: src/dojo.udl)");
        eprintln!(
            "  output_dir            Output directory for bindings (default: ../../bindings/cpp)"
        );
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --config <path>       Path to uniffi.toml config file");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {}                    # Use defaults", args[0]);
        eprintln!("  {} src/dojo.udl ../../bindings/cpp", args[0]);
        eprintln!();
        eprintln!("Requirements:");
        eprintln!("  This tool requires uniffi-bindgen-cpp to be installed:");
        eprintln!("  cd path/to/uniffi-bindgen-cpp && cargo install --path bindgen");
        eprintln!();
        eprintln!("Note:");
        eprintln!("  The compiled Rust library must be built first:");
        eprintln!("  cargo build --release");
        eprintln!();
        process::exit(0);
    }

    // Check if uniffi-bindgen-cpp is installed
    if which::which("uniffi-bindgen-cpp").is_err() {
        eprintln!("Error: uniffi-bindgen-cpp is not installed or not in PATH");
        eprintln!();
        eprintln!("Please install it with:");
        eprintln!("  cd /path/to/uniffi-bindgen-cpp");
        eprintln!("  cargo install --path bindgen");
        eprintln!();
        eprintln!("Or add it to your PATH if already installed");
        process::exit(1);
    }

    // Default paths
    let default_udl = "src/dojo.udl";
    let default_out = "../../bindings/cpp";

    // Parse arguments
    let positional_args: Vec<&String> =
        args.iter().skip(1).filter(|arg| !arg.starts_with("--")).collect();

    let udl_path =
        Utf8PathBuf::from(positional_args.first().map(|s| s.as_str()).unwrap_or(default_udl));
    let out_dir =
        Utf8PathBuf::from(positional_args.get(1).map(|s| s.as_str()).unwrap_or(default_out));

    if !udl_path.exists() {
        eprintln!("Error: UDL file not found: {udl_path}");
        eprintln!();
        eprintln!("Hint: Run with --help to see usage information");
        process::exit(1);
    }

    // Create output directory if it doesn't exist
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("Error: Failed to create output directory {out_dir}: {e}");
        process::exit(1);
    }

    println!("Generating C++ bindings for Dojo...");
    println!("UDL file: {udl_path}");
    println!("Output:   {out_dir}");

    // Build command for uniffi-bindgen-cpp
    let mut cmd = process::Command::new("uniffi-bindgen-cpp");
    cmd.arg(&udl_path);
    cmd.arg("--out-dir").arg(&out_dir);

    // Add config file if specified
    if let Some(config_idx) = args.iter().position(|arg| arg == "--config") {
        if let Some(config_path) = args.get(config_idx + 1) {
            cmd.arg("--config").arg(config_path);
        }
    } else {
        // Try default config path
        let default_config = Utf8PathBuf::from("uniffi.toml");
        if default_config.exists() {
            cmd.arg("--config").arg(&default_config);
        }
    }

    // Set working directory
    let current_dir = env::current_dir().expect("Failed to get current directory");
    cmd.current_dir(&current_dir);

    // Execute the command
    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                println!("✓ C++ bindings generated successfully!");
                println!("\nGenerated files in {out_dir}:");
                println!("  - dojo.hpp (C++ header)");
                println!("  - dojo.cpp (C++ implementation)");
                println!("  - dojo_scaffolding.hpp (FFI scaffolding)");

                if !output.stdout.is_empty() {
                    println!("\n{}", String::from_utf8_lossy(&output.stdout));
                }

                println!("\nTo use the bindings:");
                println!("  1. Include dojo.hpp in your C++ project");
                println!("  2. Link against the dojo-uniffi library");
                println!("  3. Make sure the dylib is in your library path");
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
            eprintln!("Error executing uniffi-bindgen-cpp: {e}");
            eprintln!("\nMake sure uniffi-bindgen-cpp is installed and in your PATH");
            process::exit(1);
        }
    }
}
