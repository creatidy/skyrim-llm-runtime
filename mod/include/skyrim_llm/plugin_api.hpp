#pragma once

#include <cstdint>
#include <stdbool.h>

struct SkyrimLlmPluginHandle;

extern "C" {

struct SkyrimLlmSnapshot {
    const char* game_time;
    const char* player_location;
};

using SkyrimLlmGetSnapshotFn = bool (*)(void* userdata, SkyrimLlmSnapshot* out_snapshot);
using SkyrimLlmShowStatusLineFn = void (*)(void* userdata, const char* text);
using SkyrimLlmShowMessageFn = void (*)(void* userdata, const char* title, const char* body);

struct SkyrimLlmApiConfig {
    const char* plugin_title;
    const char* requests_dir;
    const char* responses_dir;
    std::uint32_t recap_timeout_ms;
    void* userdata;
    SkyrimLlmGetSnapshotFn get_snapshot;
    SkyrimLlmShowStatusLineFn show_status_line;
    SkyrimLlmShowMessageFn show_message;
};

SkyrimLlmPluginHandle* SkyrimLlm_Create(const SkyrimLlmApiConfig* config);
void SkyrimLlm_Destroy(SkyrimLlmPluginHandle* handle);

void SkyrimLlm_RecordLocationChange(
    SkyrimLlmPluginHandle* handle,
    const char* game_time,
    const char* location);
void SkyrimLlm_RecordQuestObjective(
    SkyrimLlmPluginHandle* handle,
    const char* game_time,
    const char* objective);
void SkyrimLlm_RecordNote(
    SkyrimLlmPluginHandle* handle,
    const char* game_time,
    const char* note);
bool SkyrimLlm_TriggerHotkeyRecap(SkyrimLlmPluginHandle* handle);

}
