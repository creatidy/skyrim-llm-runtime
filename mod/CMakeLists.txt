cmake_minimum_required(VERSION 3.20)
project(skyrim_llm_mod LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_library(skyrim_llm_mod_core
    src/bridge_client.cpp
    src/controller.cpp
)
target_include_directories(skyrim_llm_mod_core PUBLIC include)

add_executable(skyrim_llm_mod_harness
    src/harness_main.cpp
)
target_link_libraries(skyrim_llm_mod_harness PRIVATE skyrim_llm_mod_core)

add_library(skyrim_llm_skse_stub STATIC
    src/skse_plugin_stub.cpp
)
target_include_directories(skyrim_llm_skse_stub PUBLIC include)
target_link_libraries(skyrim_llm_skse_stub PRIVATE skyrim_llm_mod_core)
