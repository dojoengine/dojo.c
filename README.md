# dojo.c

This package provides C bindings for the Torii Client SDK. The approach is to generate a C client using `cbindgen`.

Note:
You will need to install Rust and Cargo before running this command:

To Install Rust:
1. Go to (https://www.rust-lang.org/tools/install)

2. Download the 32-Bit / 64-Bit Installation file.

Don't know what Bit your computer is? 
**Select the Start button, then select Settings > System > About. 
At the right, under Device specifications, see System type.
It should look like:
![image](https://github.com/dojoengine/dojo.c/assets/86070833/ff73ff03-ca19-4221-b0c3-999313c63c63)


Once Rust Is Installed.
Open a new Command Prompt:
Note that Rust requires changes to the system PATH, and these changes might not take effect in the current Command Prompt window. 
To ensure that Rust commands are recognized, close the current Command Prompt window and open a new one.

Now to check if it is installed.
CTRL+R and type 'cmd' 
Once the terminal opens, inside the terming type: 
rustc --version
It should return with a value such as ![[Pasted image 20231219144818.png]]

`rustc 1.74.1 (a28077b28 2023-12-04)`

Now you can similarly see if cargo was installed by typing in terminal:
``cargo --version

This should return something like `cargo 1.74.1 (ecb9851af 2023-10-18)`

## Running

```
cargo build --release
gcc example/main.c -L target/release -l torii_c -I ..
```

## Common Errors and Fixes (WSL with WIN 10/11):

⚠️ 1. 'error: linker `cc` not found
  |
  = note: No such file or directory (os error 2)'
  \
   
    🔨 Fix: sudo apt install build-essential and run-again.

⚠️ 2. If you get the error 
error: failed to run custom build command for `torii-grpc v0.4.1 (https://github.com/dojoengine/dojo#dc5faa26)`

Caused by:
  process didn't exit successfully: `dojo.c\target\release\build\torii-grpc-263c99bc5b8fb419\build-script-build` (exit code: 101)
  --- stdout
  cargo:rerun-if-changed=proto/world.proto
  cargo:rerun-if-changed=proto

  --- stderr
  thread 'main' panicked at C:\Users\redactd\.cargo\registry\src\index.crates.io-6f17d22bba15001f\prost-build-0.12.3\src\lib.rs:1521:10:
  Could not find `protoc` installation and this build crate cannot proceed without
      this knowledge. If `protoc` is installed and this crate had trouble finding
      it, you can set the `PROTOC` environment variable with the specific path to your
      installed `protoc` binary.You can download it from https://github.com/protocolbuffers/protobuf/releases or from your package manager.

  🔨 Fix: Install proto in CLI with the commands below in cli and re-run!
$ apt install -y protobuf-compiler
$ protoc --version  # Ensure compiler version is 3+