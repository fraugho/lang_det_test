cmake_minimum_required(VERSION 3.15)
project(LangDet)

# Find Rust and Cargo
find_program(CARGO_COMMAND cargo REQUIRED)

# Define your Rust library target
set(RUST_LIB_NAME "librust_test")
set(RUST_LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../rust_test")

# Determine platform-specific extension
if(WIN32)
    set(RUST_LIB_EXT "dll")
elseif(APPLE)
    set(RUST_LIB_EXT "dylib")
else()
    set(RUST_LIB_EXT "so")
endif()

# Custom target to build Rust library
add_custom_target(
    ${RUST_LIB_NAME}_build
    COMMAND ${CARGO_COMMAND} build --release
    WORKING_DIRECTORY ${RUST_LIB_DIR}
    COMMENT "Building Rust library"
)

# Import the built library
add_library(${RUST_LIB_NAME} SHARED IMPORTED)
message(STATUS "Rust lib path: ${RUST_LIB_DIR}/target/release/${RUST_LIB_NAME}.${RUST_LIB_EXT}")
set_target_properties(${RUST_LIB_NAME} PROPERTIES
    IMPORTED_LOCATION "${RUST_LIB_DIR}/target/release/${RUST_LIB_NAME}.${RUST_LIB_EXT}"
)

# Your C++ executable
add_executable(m main.cpp)
add_dependencies(m ${RUST_LIB_NAME}_build)
target_link_libraries(m ${RUST_LIB_NAME})

# Link system libraries that Rust needs
if(WIN32)
    target_link_libraries(m ws2_32 userenv advapi32)
endif()
