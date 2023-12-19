# dojo.c

This package provides C bindings for the Torii Client SDK. The approach is to generate a C client using `cbindgen`.

Note:
You will need to install Rust and Cargo before running this command:

To Install `1 Rust`:
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
