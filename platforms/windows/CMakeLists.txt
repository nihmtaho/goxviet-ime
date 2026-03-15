cmake_minimum_required(VERSION 3.22)
project(goxviet-windows LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

option(ENABLE_DEBUG_CONSOLE "Enable debug console window" OFF)

if(MSVC)
    set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")
endif()

set(SOURCES
    # Entry point
    src/main.cpp

    # Core engine bridge
    src/rust_bridge.cpp

    # Event system
    src/notifications.cpp

    # Settings (data model + SRP persistence layer)
    src/settings_store.cpp
    src/settings.cpp

    # Input pipeline
    src/vk_mapper.cpp
    src/keyboard_hook.cpp
    src/restore_shortcut.cpp

    # Compatibility & per-app
    src/app_compat.cpp
    src/per_app.cpp
    src/input_language_monitor.cpp

    # UI
    src/system_tray.cpp
    src/settings_window.cpp
    src/modern_ui.cpp

    # Utilities
    src/utils.cpp
    src/debug_console.cpp

    # Resources
    resources/resources.rc
)

if(ENABLE_DEBUG_CONSOLE)
    add_executable(goxviet ${SOURCES})
    target_compile_definitions(goxviet PRIVATE GOXVIET_DEBUG_CONSOLE)
else()
    add_executable(goxviet WIN32 ${SOURCES})
endif()

target_link_libraries(goxviet PRIVATE
    user32 shell32 gdi32 comctl32 psapi
    dwmapi gdiplus uxtheme ole32 comdlg32
    ws2_32 advapi32 shlwapi
)

target_include_directories(goxviet PRIVATE ${CMAKE_SOURCE_DIR}/src)

target_compile_definitions(goxviet PRIVATE UNICODE _UNICODE)

if(MSVC)
    target_link_options(goxviet PRIVATE /MANIFEST:NO)
    # Treat all source files as UTF-8 — required for Vietnamese string literals
    target_compile_options(goxviet PRIVATE /utf-8)
endif()

if(MSVC)
    target_compile_options(goxviet PRIVATE
        $<$<NOT:$<CONFIG:Debug>>:/O1 /GL /GS->
    )
    target_link_options(goxviet PRIVATE
        $<$<NOT:$<CONFIG:Debug>>:/LTCG /OPT:REF /OPT:ICF>
    )
endif()

# ── Copy goxviet_core.dll next to the executable after every build ──────────
# GOXVIET_RUST_TARGET and GOXVIET_RUST_PROFILE are set per-config in
# CMakeSettings.json (Ninja) or passed on the command line for CI.

if(NOT DEFINED GOXVIET_RUST_TARGET)
    # Fallback: guess from MSVC target architecture
    if(CMAKE_SIZEOF_VOID_P EQUAL 4)
        set(GOXVIET_RUST_TARGET "i686-pc-windows-msvc")
    elseif(CMAKE_SYSTEM_PROCESSOR MATCHES "ARM64|aarch64")
        set(GOXVIET_RUST_TARGET "aarch64-pc-windows-msvc")
    else()
        set(GOXVIET_RUST_TARGET "x86_64-pc-windows-msvc")
    endif()
endif()

if(NOT DEFINED GOXVIET_RUST_PROFILE)
    set(GOXVIET_RUST_PROFILE "$<IF:$<CONFIG:Debug>,debug,release>")
endif()

set(RUST_DLL_SRC
    "${CMAKE_SOURCE_DIR}/../../core/target/${GOXVIET_RUST_TARGET}/${GOXVIET_RUST_PROFILE}/goxviet_core.dll")
set(RUST_DLL_FALLBACK "${CMAKE_SOURCE_DIR}/goxviet_core.dll")

add_custom_command(TARGET goxviet POST_BUILD
    COMMAND ${CMAKE_COMMAND}
        -DSRC1=${RUST_DLL_SRC}
        -DSRC2=${RUST_DLL_FALLBACK}
        -DDST=$<TARGET_FILE_DIR:goxviet>
        -P "${CMAKE_SOURCE_DIR}/cmake/copy_dll.cmake"
    COMMENT "Copying goxviet_core.dll → output dir"
    VERBATIM
)
