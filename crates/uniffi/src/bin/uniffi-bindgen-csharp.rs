use std::{env, process};

use camino::Utf8PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Show help if requested
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        eprintln!("UniFFI C# Binding Generator");
        eprintln!();
        eprintln!("Usage: {} [library_path] [output_dir] [OPTIONS]", args[0]);
        eprintln!();
        eprintln!("Arguments:");
        eprintln!(
            "  library_path          Path to the compiled library (default: \
             target/release/libdojo_uniffi.dylib)"
        );
        eprintln!(
            "  output_dir            Output directory for bindings (default: bindings/csharp)"
        );
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --config <path>       Path to uniffi.toml config file");
        eprintln!("  --library             Generate library mode bindings");
        eprintln!("  --no-format           Skip code formatting");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {}                    # Use defaults", args[0]);
        eprintln!("  {} target/release/libdojo_uniffi.dylib bindings/csharp", args[0]);
        eprintln!(
            "  {} target/release/libdojo_uniffi.dylib bindings/csharp --config \
             crates/uniffi/uniffi.toml",
            args[0]
        );
        eprintln!();
        eprintln!("Requirements:");
        eprintln!("  This tool requires uniffi-bindgen-cs to be installed:");
        eprintln!("  cargo install uniffi-bindgen-cs --git https://github.com/NordSecurity/uniffi-bindgen-cs --tag v0.10.0+v0.29.4");
        eprintln!();
        process::exit(0);
    }

    // Check if uniffi-bindgen-cs is installed
    if which::which("uniffi-bindgen-cs").is_err() {
        eprintln!("Error: uniffi-bindgen-cs is not installed or not in PATH");
        eprintln!();
        eprintln!("Please install it with:");
        eprintln!("  cargo install uniffi-bindgen-cs --git https://github.com/NordSecurity/uniffi-bindgen-cs --tag v0.10.0+v0.29.4");
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

    // Default paths (must match the library output name)
    let default_lib = format!("target/release/libdojo_uniffi.{lib_ext}");
    let default_out = "bindings/csharp";

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

    println!("Generating C# bindings...");
    println!("Library: {library_path}");
    println!("Output:  {out_dir}");

    // Build command for uniffi-bindgen-cs
    let mut cmd = process::Command::new("uniffi-bindgen-cs");
    cmd.arg(&library_path);
    cmd.arg("--library"); // Always use --library flag when passing library path
    cmd.arg("--out-dir").arg(&out_dir);

    if args.contains(&"--no-format".to_string()) {
        cmd.arg("--no-format");
    }

    // Add config file if specified
    if let Some(config_idx) = args.iter().position(|arg| arg == "--config") {
        if let Some(config_path) = args.get(config_idx + 1) {
            cmd.arg("--config").arg(config_path);
        }
    } else {
        // Try default config path
        let default_config = Utf8PathBuf::from("crates/uniffi/uniffi.toml");
        if default_config.exists() {
            cmd.arg("--config").arg(&default_config);
        }
    }

    // Execute the command
    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                println!("✓ C# bindings generated successfully!");
                if !output.stdout.is_empty() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
            } else {
                eprintln!("Error generating bindings:");
                if !output.stderr.is_empty() {
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error executing uniffi-bindgen-cs: {e}");
            process::exit(1);
        }
    }
}
