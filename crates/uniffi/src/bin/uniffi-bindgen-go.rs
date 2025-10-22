use std::{env, process};

use camino::Utf8PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Show help if requested
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        eprintln!("UniFFI Go Binding Generator");
        eprintln!();
        eprintln!("Usage: {} [udl_path] [output_dir] [OPTIONS]", args[0]);
        eprintln!();
        eprintln!("Arguments:");
        eprintln!(
            "  udl_path              Path to the .udl file (default: crates/uniffi/src/dojo.udl)"
        );
        eprintln!("  output_dir            Output directory for bindings (default: bindings/go)");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --config <path>       Path to uniffi.toml config file");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  {}                    # Use defaults", args[0]);
        eprintln!("  {} crates/uniffi/src/dojo.udl bindings/go", args[0]);
        eprintln!(
            "  {} crates/uniffi/src/dojo.udl bindings/go --config crates/uniffi/uniffi.toml",
            args[0]
        );
        eprintln!();
        eprintln!("Requirements:");
        eprintln!("  This tool requires uniffi-bindgen-go to be installed:");
        eprintln!("  cargo install uniffi-bindgen-go --git https://github.com/NordSecurity/uniffi-bindgen-go --tag v0.4.0+v0.28.3");
        eprintln!();
        eprintln!("Note:");
        eprintln!("  The compiled Rust library must be in your LD_LIBRARY_PATH");
        eprintln!("  For development, set: export LD_LIBRARY_PATH=target/release");
        eprintln!();
        process::exit(0);
    }

    // Check if uniffi-bindgen-go is installed
    if which::which("uniffi-bindgen-go").is_err() {
        eprintln!("Error: uniffi-bindgen-go is not installed or not in PATH");
        eprintln!();
        eprintln!("Please install it with:");
        eprintln!("  cargo install uniffi-bindgen-go --git https://github.com/NordSecurity/uniffi-bindgen-go --tag v0.4.0+v0.28.3");
        eprintln!();
        eprintln!("Or add it to your PATH if already installed");
        process::exit(1);
    }

    // Default paths
    let default_udl = "crates/uniffi/src/dojo.udl";
    let default_out = "bindings/go";

    // Parse arguments
    let positional_args: Vec<&String> =
        args.iter().skip(1).filter(|arg| !arg.starts_with("--")).collect();

    let udl_path =
        Utf8PathBuf::from(positional_args.first().map(|s| s.as_str()).unwrap_or(default_udl));
    let out_dir =
        Utf8PathBuf::from(positional_args.get(1).map(|s| s.as_str()).unwrap_or(default_out));

    if !udl_path.exists() {
        eprintln!("Error: UDL file not found: {}", udl_path);
        eprintln!();
        eprintln!("Hint: Run with --help to see usage information");
        process::exit(1);
    }

    // Create output directory if it doesn't exist
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("Error: Failed to create output directory {}: {}", out_dir, e);
        process::exit(1);
    }

    println!("Generating Go bindings...");
    println!("UDL file: {}", udl_path);
    println!("Output:   {}", out_dir);

    // Build command for uniffi-bindgen-go
    let mut cmd = process::Command::new("uniffi-bindgen-go");
    cmd.arg(&udl_path);

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

    // Set working directory to the output directory's parent
    // uniffi-bindgen-go generates files relative to the UDL location
    let current_dir = env::current_dir().expect("Failed to get current directory");
    cmd.current_dir(&current_dir);

    // Execute the command
    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                // uniffi-bindgen-go generates files in a specific structure
                // The output will be in uniffi/<namespace>/<namespace>.go
                println!("✓ Go bindings generated successfully!");
                println!("\nNote: Bindings are generated relative to the UDL file location.");
                println!("Look for the generated .go files in the uniffi subdirectory.");

                if !output.stdout.is_empty() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }

                // Try to copy the generated files to the desired output directory
                // uniffi-bindgen-go creates files in <namespace>/<namespace>.go relative to UDL
                let udl_dir = udl_path.parent().unwrap();
                let dojo_dir = udl_dir.join("dojo"); // namespace is "dojo"

                if dojo_dir.exists() {
                    println!("\nCopying generated files to {}...", out_dir);
                    if let Err(e) = copy_dir_recursive(&dojo_dir, &out_dir) {
                        eprintln!("Warning: Failed to copy files: {}", e);
                        eprintln!("You may need to manually copy files from {}", dojo_dir);
                    } else {
                        println!("✓ Files copied to {}", out_dir);
                        // Clean up the generated directory in src
                        let _ = std::fs::remove_dir_all(&dojo_dir);
                    }
                } else {
                    eprintln!("Warning: Generated Go files not found at {}", dojo_dir);
                    eprintln!("They may be in a different location.");
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
            eprintln!("Error executing uniffi-bindgen-go: {}", e);
            process::exit(1);
        }
    }
}

fn copy_dir_recursive(src: &Utf8PathBuf, dst: &Utf8PathBuf) -> std::io::Result<()> {
    use std::fs;

    use walkdir::WalkDir;

    for entry in WalkDir::new(src.as_str()) {
        let entry = entry?;
        let path = entry.path();

        let relative_path = path.strip_prefix(src.as_str()).unwrap();
        let target_path = dst.join(relative_path.to_str().unwrap());

        if entry.file_type().is_dir() {
            fs::create_dir_all(&target_path)?;
        } else {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &target_path)?;
        }
    }

    Ok(())
}
