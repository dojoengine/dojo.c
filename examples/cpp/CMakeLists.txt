cmake_minimum_required(VERSION 3.20)
project(dojo_cpp_examples)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add the fetch_entities executable
add_executable(fetch_entities
    fetch_entities.cpp
    ../../bindings/cpp/dojo.cpp
)

# Include directories
target_include_directories(fetch_entities PRIVATE
    ../../bindings/cpp
)

# Link against the dojo library
if(APPLE)
    target_link_libraries(fetch_entities
        ${CMAKE_SOURCE_DIR}/../../target/release/libdojo_uniffi.dylib
    )
    # Set rpath for macOS
    set_target_properties(fetch_entities PROPERTIES
        BUILD_RPATH "${CMAKE_SOURCE_DIR}/../../target/release"
        INSTALL_RPATH "${CMAKE_SOURCE_DIR}/../../target/release"
    )
elseif(UNIX)
    target_link_libraries(fetch_entities
        ${CMAKE_SOURCE_DIR}/../../target/release/libdojo_uniffi.so
    )
    # Set rpath for Linux
    set_target_properties(fetch_entities PROPERTIES
        BUILD_RPATH "${CMAKE_SOURCE_DIR}/../../target/release"
        INSTALL_RPATH "${CMAKE_SOURCE_DIR}/../../target/release"
    )
elseif(WIN32)
    target_link_libraries(fetch_entities
        ${CMAKE_SOURCE_DIR}/../../target/release/dojo_uniffi.dll
    )
endif()


