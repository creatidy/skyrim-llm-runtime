#include "skyrim_llm_skse_host/hotkey_binding.hpp"

#if __has_include(<RE/Skyrim.h>)
#include <RE/Skyrim.h>
#define SKYRIM_LLM_HAS_RE_API 1
#else
#define SKYRIM_LLM_HAS_RE_API 0
#endif

namespace skyrim_llm::skse_host {
namespace {

struct ListenerState {
    std::function<void()> handler;
    std::uint32_t keyboard_dx_scan_code{RecapHotkeyBinding::kDefaultKeyboardDxScanCode};
};

}  // namespace

struct RecapHotkeyBinding::Listener {
    explicit Listener(ListenerState state) : state_(std::move(state)) {}

    bool HandleKeyEvent(std::uint32_t scan_code, bool pressed) const {
        if (!pressed || scan_code != state_.keyboard_dx_scan_code || state_.handler == nullptr) {
            return false;
        }

        state_.handler();
        return true;
    }

#if SKYRIM_LLM_HAS_RE_API
    bool Install() {
        // TODO(skyrim-phase2): Register this listener with the chosen Skyrim
        // input pipeline. Typical options are:
        // - a BSInputDeviceManager/BSInputEventReceiver sink
        // - an SKSE input event registration path
        // - a CommonLibSSE-NG event sink helper
        //
        // Once installed, Skyrim keyboard button events should be forwarded to
        // HandleKeyEvent(scan_code, pressed).
        return true;
    }
#else
    bool Install() {
        return true;
    }
#endif

    ListenerState state_;
};

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

    listener_ = std::make_shared<Listener>(ListenerState{
        .handler = handler_,
        .keyboard_dx_scan_code = keyboard_dx_scan_code_,
    });
    registered_ = InstallSkyrimInputHook();
    return registered_;
}

bool RecapHotkeyBinding::IsRegistered() const {
    return registered_;
}

bool RecapHotkeyBinding::FeedKeyEvent(std::uint32_t scan_code, bool pressed) const {
    if (!listener_ || !MatchesRegisteredHotkey(scan_code, pressed)) {
        return false;
    }
    return listener_->HandleKeyEvent(scan_code, pressed);
}

void RecapHotkeyBinding::DispatchForTesting() const {
    if (handler_ != nullptr) {
        handler_();
    }
}

bool RecapHotkeyBinding::InstallSkyrimInputHook() {
    if (!listener_) {
        return false;
    }
    return listener_->Install();
}

bool RecapHotkeyBinding::MatchesRegisteredHotkey(std::uint32_t scan_code, bool pressed) const {
    return pressed && scan_code == keyboard_dx_scan_code_;
}

}  // namespace skyrim_llm::skse_host
