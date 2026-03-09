cmake_minimum_required(VERSION 3.20)
project(skyrim_llm_skse_host LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

option(SKYRIM_LLM_BUILD_WINDOWS_PLUGIN "Build the Windows Skyrim plugin target when dependencies are available" ON)
set(SKYRIM_LLM_DEPLOY_DIR "" CACHE PATH "Optional deployment directory for the built Skyrim plugin DLL/PDB")

set(SKYRIM_LLM_MOD_SOURCES
    ../mod/src/bridge_client.cpp
    ../mod/src/controller.cpp
    ../mod/src/plugin_api.cpp
    ../mod/src/skse_plugin_stub.cpp
    ../mod/src/skyrim_integration.cpp
)

set(SKYRIM_LLM_HOST_SOURCES
    src/game_time_capture.cpp
    src/host_context.cpp
    src/hotkey.cpp
    src/hotkey_binding.cpp
    src/location_capture.cpp
    src/notification_ui.cpp
    src/plugin_main.cpp
    src/plugin_shell.cpp
    src/snapshot.cpp
    src/skyrim_game_api.cpp
    src/skyrim_ui_api.cpp
    src/ui.cpp
)

add_library(skyrim_llm_mod_runtime_support STATIC
    ${SKYRIM_LLM_MOD_SOURCES}
)
target_include_directories(skyrim_llm_mod_runtime_support
    PUBLIC
        ../mod/include
)

add_library(skyrim_llm_skse_host_scaffold STATIC
    ${SKYRIM_LLM_HOST_SOURCES}
)
target_include_directories(skyrim_llm_skse_host_scaffold
    PUBLIC
        include
        ../mod/include
)
target_link_libraries(skyrim_llm_skse_host_scaffold
    PUBLIC
        skyrim_llm_mod_runtime_support
)

if(MSVC)
    target_compile_options(skyrim_llm_mod_runtime_support PRIVATE /W4 /permissive- /EHsc /utf-8)
    target_compile_options(skyrim_llm_skse_host_scaffold PRIVATE /W4 /permissive- /EHsc /utf-8)
else()
    target_compile_options(skyrim_llm_mod_runtime_support PRIVATE -Wall -Wextra -Wpedantic)
    target_compile_options(skyrim_llm_skse_host_scaffold PRIVATE -Wall -Wextra -Wpedantic)
endif()

if(WIN32 AND SKYRIM_LLM_BUILD_WINDOWS_PLUGIN)
    find_package(CommonLibSSE CONFIG QUIET)

    if(CommonLibSSE_FOUND)
        add_library(skyrim_llm_skse_plugin SHARED
            ${SKYRIM_LLM_HOST_SOURCES}
        )
        target_include_directories(skyrim_llm_skse_plugin
            PRIVATE
                include
                ../mod/include
        )
        target_link_libraries(skyrim_llm_skse_plugin
            PRIVATE
                skyrim_llm_mod_runtime_support
                CommonLibSSE::CommonLibSSE
        )
        set_target_properties(skyrim_llm_skse_plugin PROPERTIES
            OUTPUT_NAME "SkyrimLLMRuntime"
            PREFIX ""
        )

        if(MSVC)
            target_compile_options(skyrim_llm_skse_plugin PRIVATE /W4 /permissive- /EHsc /utf-8)
        else()
            target_compile_options(skyrim_llm_skse_plugin PRIVATE -Wall -Wextra -Wpedantic)
        endif()

        if(SKYRIM_LLM_DEPLOY_DIR)
            add_custom_command(TARGET skyrim_llm_skse_plugin POST_BUILD
                COMMAND ${CMAKE_COMMAND} -E make_directory "${SKYRIM_LLM_DEPLOY_DIR}"
                COMMAND ${CMAKE_COMMAND} -E copy_if_different
                    "$<TARGET_FILE:skyrim_llm_skse_plugin>"
                    "${SKYRIM_LLM_DEPLOY_DIR}/$<TARGET_FILE_NAME:skyrim_llm_skse_plugin>"
                VERBATIM
            )

            if(MSVC)
                add_custom_command(TARGET skyrim_llm_skse_plugin POST_BUILD
                    COMMAND ${CMAKE_COMMAND} -E copy_if_different
                        "$<TARGET_PDB_FILE:skyrim_llm_skse_plugin>"
                        "${SKYRIM_LLM_DEPLOY_DIR}/$<TARGET_PDB_FILE_NAME:skyrim_llm_skse_plugin>"
                    VERBATIM
                    COMMAND_EXPAND_LISTS
                )
            endif()
        endif()
    else()
        message(STATUS "CommonLibSSE not found; building only skyrim_llm_skse_host_scaffold")
    endif()
endif()

# The portable scaffold target stays buildable in this repo even when Windows-only
# SKSE/CommonLibSSE dependencies are not installed. The real plugin DLL target is
# added only for Windows builds where CommonLibSSE is available.
