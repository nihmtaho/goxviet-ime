# copy_dll.cmake — post-build helper
# Called by CMakeLists.txt with:
#   -DSRC1=<rust-build-output-dll>
#   -DSRC2=<project-root-dll>
#   -DDST=<target-file-dir>

if(EXISTS "${SRC1}")
    file(COPY "${SRC1}" DESTINATION "${DST}")
    message(STATUS "Copied DLL from Rust build: ${SRC1}")
elseif(EXISTS "${SRC2}")
    file(COPY "${SRC2}" DESTINATION "${DST}")
    message(STATUS "Copied DLL from project root: ${SRC2}")
else()
    message(WARNING
        "goxviet_core.dll not found.\n"
        "  Checked: ${SRC1}\n"
        "  Checked: ${SRC2}\n"
        "Build the Rust core first:\n"
        "  cd core && cargo build --release --target aarch64-pc-windows-msvc")
endif()
