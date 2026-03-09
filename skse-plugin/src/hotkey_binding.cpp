#include "skyrim_llm_skse_host/hotkey_binding.hpp"

#if __has_include(<RE/Skyrim.h>)
#include <RE/Skyrim.h>
#define SKYRIM_LLM_HAS_RE_API 1
#else
#define SKYRIM_LLM_HAS_RE_API 0
#endif

namespace skyrim_llm::skse_host {

void RecapHotkeyBinding::SetHandler(std::function<void()> handler) {
    handler_ = std::move(handler);
}

void RecapHotkeyBinding::SetKeyboardDxScanCode(std::uint32_t scan_code) {
    keyboard_dx_scan_code_ = scan_code;
}

bool RecapHotkeyBinding::RegisterDefaultHotkey() {
    if (handler_ == nullptr) {
        registered_ = false;
        return false;
    }

    registered_ = InstallSkyrimInputHook();
    return registered_;
}

bool RecapHotkeyBinding::IsRegistered() const {
    return registered_;
}

void RecapHotkeyBinding::DispatchForTesting() const {
    if (handler_ != nullptr) {
        handler_();
    }
}

bool RecapHotkeyBinding::InstallSkyrimInputHook() {
#if SKYRIM_LLM_HAS_RE_API
    // TODO(skyrim-phase2): Install a real keyboard input listener through the
    // chosen SKSE/CommonLibSSE path. A practical implementation usually does one
    // of the following:
    // 1. register an input event sink and inspect RE::ButtonEvent values, or
    // 2. hook an existing menu/input handler and dispatch into handler_ when the
    //    registered DirectInput scan code is pressed.
    //
    // Expected matching logic:
    // - keyboard device only
    // - scan code == keyboard_dx_scan_code_
    // - pressed event transition only
    //
    // When that integration is added, it should call handler_() on match and
    // return true once the sink/hook is successfully installed.
    return true;
#else
    // Outside the real Windows Skyrim build we keep the scaffold usable without
    // forcing Skyrim headers or dependencies.
    return true;
#endif
}

bool RecapHotkeyBinding::MatchesRegisteredHotkey(std::uint32_t scan_code, bool pressed) const {
    return pressed && scan_code == keyboard_dx_scan_code_;
}

}  // namespace skyrim_llm::skse_host
