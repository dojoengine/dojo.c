# C++ Examples for Dojo

## Current Status

⚠️ **Important Note**: The C++ bindings for `dojo.c` currently have compilation issues due to recursive type definitions in the generated header files. This is a known limitation of `uniffi-bindgen-cpp` when dealing with complex, self-referential data structures like `Clause`, `Ty`, `MemberValue`, etc.

The generated `dojo.hpp` contains forward declarations that are used in `std::optional<>` before their full definitions, causing C++ compilation errors.

## ✅ Working C++ Bindings

**Good news**: The C++ bindings for `controller.c` work perfectly! If you need C++ bindings for Dojo functionality, please use:

### Controller C++ Example (Fully Working!)
```bash
cd ../../controller.c/examples/cpp
./run.sh
```

This comprehensive test suite demonstrates:
- ✅ Owner creation
- ✅ Controller creation and management
- ✅ Transaction execution
- ✅ Session accounts
- ✅ Error handling
- ✅ All controller functionality

See: [`controller.c/examples/cpp/controller_test.cpp`](../../controller.c/examples/cpp/controller_test.cpp) for a complete working example.

## 📚 Alternative Bindings for Dojo.c

For full Torii client functionality with entity queries, subscriptions, and complex operations, please use these fully supported bindings:

### Swift Bindings (Recommended)
```bash
cd ../swift
swift fetch_entities.swift http://localhost:8080
```

Features:
- ✅ Full ToriiClient support
- ✅ Entity queries with complex clauses
- ✅ Subscriptions
- ✅ All Dojo functionality
- ✅ Excellent type safety and performance

### Python Bindings
```bash
cd ../python
python3 fetch_entities.py
```

Features:
- ✅ Complete API coverage
- ✅ Easy to use
- ✅ Great for scripting and automation

### Kotlin Bindings
```bash
cd ../kotlin
./run_fetch_entities.sh
```

Features:
- ✅ Android-ready
- ✅ JVM interoperability
- ✅ Full API support

## Technical Details

### The Issue

The generated C++ bindings have this structure in `dojo.hpp`:

```cpp
// Forward declarations
struct Clause;
struct Ty;
struct MemberValue;

// Later in the file...
struct Query {
    std::optional<Clause> clause;  // ❌ Error: incomplete type
    // ...
};

struct Clause {
    // Recursive definition involving Ty, MemberValue, etc.
};
```

The C++ compiler cannot instantiate `std::optional<Clause>` before `Clause` is fully defined, especially when `Clause` is recursive.

### Why Controller.c Works

The `controller.c` project has simpler, non-recursive types:
- `Owner`, `Controller`, `SessionAccount` - all non-recursive
- `Call`, `SessionPolicies` - simple struct compositions
- No self-referential or mutually recursive types

This allows `uniffi-bindgen-cpp` to generate clean, compilable C++ code.

### Potential Solutions

1. **Fix uniffi-bindgen-cpp** (Complex)
   - Modify the code generator to handle recursive types
   - Use forward declarations with `unique_ptr` or `shared_ptr`
   - Reorganize header file generation order

2. **Simplify Dojo UDL** (Not practical)
   - Would require major architectural changes
   - Would lose functionality

3. **Use Other Language Bindings** (Recommended)
   - Swift, Python, Kotlin all work perfectly
   - These languages handle recursive types naturally

## Examples Directory Structure

```
examples/cpp/
├── README.md                  # This file
├── fetch_entities.cpp         # ⚠️  Currently non-functional
├── CMakeLists.txt            # Build configuration
├── build.sh                  # Build script
└── run_fetch_entities.sh     # Run script
```

## C++ Support Summary

| Project | Status | Notes |
|---------|--------|-------|
| **controller.c** | ✅ Fully Working | Simple, non-recursive types |
| **dojo.c** | ❌ Compilation Errors | Complex recursive types |

## Recommendations

1. **For Controller Operations**: Use C++ bindings ✅
   - See `controller.c/examples/cpp/`
   - Full test suite available
   - Production-ready

2. **For Torii/Entity Operations**: Use other languages ✅
   - Swift (recommended for Apple platforms)
   - Python (recommended for scripting)
   - Kotlin (recommended for JVM/Android)
   - Go (also available)
   - C# (also available)

3. **For Mixed Usage**: Combine C++ controller with FFI
   - Use C++ for controller operations
   - Use Python/Swift for Torii queries
   - Bridge between them as needed

## Future Work

Potential improvements for C++ support:

- [ ] Fix uniffi-bindgen-cpp recursive type handling
- [ ] Contribute upstream improvements
- [ ] Add wrapper layer to hide recursive types
- [ ] Create simplified C++ API facade

## More Information

- **Working C++ Example**: [`controller.c/examples/cpp/controller_test.cpp`](../../controller.c/examples/cpp/controller_test.cpp)
- **Swift Example**: [`examples/swift/fetch_entities.swift`](../swift/fetch_entities.swift)
- **Python Example**: [`examples/python/fetch_entities.py`](../python/fetch_entities.py)
- **Dojo Documentation**: https://book.dojoengine.org
- **UniFFI**: https://mozilla.github.io/uniffi-rs/

---

**Status**: C++ bindings for `controller.c` are fully functional ✅  
**Alternative**: Use Swift/Python/Kotlin for `dojo.c` functionality ✅  
**Last Updated**: 2025-10-23


