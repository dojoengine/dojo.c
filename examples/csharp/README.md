# Dojo C# Examples

This directory contains examples demonstrating how to use the Dojo C# bindings.

## Prerequisites

- .NET SDK 8.0 or higher (or .NET Framework 4.6.1+)
- Dojo uniffi library built (`cargo build --release -p dojo-uniffi`)
- Generated C# bindings (see below)

## Generating Bindings

Before running the examples, you need to generate the C# bindings:

```bash
# From the repository root
./scripts/build_csharp.sh
```

This will:
1. Build the Rust library
2. Install `uniffi-bindgen-cs` if not already installed
3. Generate C# bindings in `bindings/csharp/`

## Project Requirements

Your C# project must:

1. **Enable unsafe code** in your `.csproj`:
   ```xml
   <PropertyGroup>
       <AllowUnsafeBlocks>true</AllowUnsafeBlocks>
   </PropertyGroup>
   ```

2. **Target appropriate framework**:
   - .NET 8.0 or higher (recommended)
   - .NET Framework 4.6.1 or higher (requires additional packages)

3. **Include the generated bindings**:
   - Copy the generated `.cs` file to your project, or
   - Reference it directly from `bindings/csharp/`

## Running the Examples

### Fetch Entities Example

This example shows how to connect to a Torii server and fetch entities.

```bash
# Navigate to the examples directory
cd examples/csharp

# Make sure the native library is accessible
# On macOS/Linux:
export DYLD_LIBRARY_PATH=../../target/release:$DYLD_LIBRARY_PATH  # macOS
export LD_LIBRARY_PATH=../../target/release:$LD_LIBRARY_PATH      # Linux

# On Windows, copy the DLL to the output directory or add to PATH

# Run the example
dotnet run

# Or with custom Torii URL and world address
dotnet run http://localhost:8080 0x1234...
```

## Using in Your Own Project

1. **Copy the generated bindings**:
   ```bash
   cp ../../bindings/csharp/*.cs YourProject/
   ```

2. **Configure your `.csproj`**:
   ```xml
   <Project Sdk="Microsoft.NET.Sdk">
     <PropertyGroup>
       <TargetFramework>net8.0</TargetFramework>
       <AllowUnsafeBlocks>true</AllowUnsafeBlocks>
     </PropertyGroup>
   </Project>
   ```

3. **Ensure native library is accessible**:
   - Copy `libdojo_uniffi.dylib` (macOS), `libdojo_uniffi.so` (Linux), or `dojo_uniffi.dll` (Windows) to your output directory
   - Or set the appropriate environment variable (`DYLD_LIBRARY_PATH`, `LD_LIBRARY_PATH`, or `PATH`)

4. **Use the bindings**:
   ```csharp
   using DojoEngine;
   
   var client = await ToriiClient.NewWithConfig("http://localhost:8080", 4 * 1024 * 1024);
   var page = await client.Entities(query);
   ```

## .NET Framework 4.6.1 Support

If you need to target .NET Framework 4.6.1, add these packages to your `.csproj`:

```xml
<PropertyGroup>
  <TargetFramework>net461</TargetFramework>
  <LangVersion>10.0</LangVersion>
  <AllowUnsafeBlocks>true</AllowUnsafeBlocks>
</PropertyGroup>

<ItemGroup>
  <PackageReference Include="IsExternalInit" Version="1.0.3"/>
  <PackageReference Include="Microsoft.CSharp" Version="4.7.0" />
</ItemGroup>
```

## Troubleshooting

### "Could not load library" errors

Make sure the native library is in your library path:
- **macOS**: `export DYLD_LIBRARY_PATH=/path/to/target/release`
- **Linux**: `export LD_LIBRARY_PATH=/path/to/target/release`
- **Windows**: Copy the DLL to your exe directory or add to PATH

### Compilation errors

- Ensure `AllowUnsafeBlocks` is set to `true`
- Check that you're targeting a compatible .NET version
- Verify the bindings were generated successfully

### Runtime errors

- Make sure the Torii server is running and accessible
- Check that the world address is correct
- Verify network connectivity

## Available Examples

- **FetchEntities.cs** - Demonstrates connecting to Torii and fetching entities with pagination

## More Information

For more details on the Dojo C# bindings:
- UniFFI C# bindings: https://github.com/NordSecurity/uniffi-bindgen-cs
- Dojo documentation: https://book.dojoengine.org/

