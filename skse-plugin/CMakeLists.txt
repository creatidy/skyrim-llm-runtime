cmake_minimum_required(VERSION 3.20)
project(skyrim_llm_skse_host LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_library(skyrim_llm_skse_host_scaffold STATIC
    src/host_context.cpp
    src/hotkey.cpp
    src/plugin_main.cpp
    src/plugin_shell.cpp
    src/snapshot.cpp
    src/skyrim_game_api.cpp
    src/skyrim_ui_api.cpp
    src/ui.cpp
)

target_include_directories(skyrim_llm_skse_host_scaffold
    PUBLIC
        include
        ../mod/include
)

# This target is intentionally only the thin host scaffold. A real Skyrim-loadable
# DLL still needs Windows-side SKSE/CommonLibSSE-NG integration and an actual
# plugin entrypoint in a Windows build environment.
