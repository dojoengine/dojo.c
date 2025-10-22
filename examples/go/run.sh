#!/bin/bash
# Helper script to run Go examples with proper library path set

set -e

# Determine the platform
case "$(uname -s)" in
    Darwin*)
        export DYLD_LIBRARY_PATH="$PWD/../../target/release:${DYLD_LIBRARY_PATH}"
        echo "Set DYLD_LIBRARY_PATH for macOS"
        ;;
    Linux*)
        export LD_LIBRARY_PATH="$PWD/../../target/release:${LD_LIBRARY_PATH}"
        echo "Set LD_LIBRARY_PATH for Linux"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        echo "Windows detected - make sure dojo_uniffi.dll is in PATH or current directory"
        ;;
    *)
        echo "Unknown platform: $(uname -s)"
        ;;
esac

# Run the Go command with all arguments passed to this script
echo "Running: go $@"
exec go "$@"

