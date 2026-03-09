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
    Listener(const RecapHotkeyBinding& owner, ListenerState state)
        : owner_(owner), state_(std::move(state)) {}

    bool HandleKeyEvent(std::uint32_t scan_code, bool pressed) const {
        if (!pressed || scan_code != state_.keyboard_dx_scan_code || state_.handler == nullptr) {
            return false;
        }

        state_.handler();
        return true;
    }

#if SKYRIM_LLM_HAS_RE_API
    class SkyrimInputEventSink final : public RE::BSTEventSink<RE::InputEvent*> {
    public:
        explicit SkyrimInputEventSink(const Listener& owner) : owner_(owner) {}

        RE::BSEventNotifyControl ProcessEvent(
            RE::InputEvent* const* events,
            RE::BSTEventSource<RE::InputEvent*>*) override {
            if (events == nullptr) {
                return RE::BSEventNotifyControl::kContinue;
            }

            for (auto* event = *events; event != nullptr; event = event->next) {
                if (event->GetDevice() != RE::INPUT_DEVICE::kKeyboard) {
                    continue;
                }

                const auto* button = event->AsButtonEvent();
                if (button == nullptr) {
                    continue;
                }

                owner_.owner_.FeedKeyEvent(button->idCode, button->IsPressed());
            }

            return RE::BSEventNotifyControl::kContinue;
        }

    private:
        const Listener& owner_;
    };

    bool Install() {
        if (installed_) {
            return true;
        }

        auto* input_manager = RE::BSInputDeviceManager::GetSingleton();
        if (input_manager == nullptr) {
            return false;
        }

        sink_ = std::make_unique<SkyrimInputEventSink>(*this);
        input_manager->AddEventSink(sink_.get());
        installed_ = true;
        return true;
    }

    void Uninstall() {
        if (!installed_) {
            return;
        }

        if (auto* input_manager = RE::BSInputDeviceManager::GetSingleton(); input_manager != nullptr && sink_) {
            input_manager->RemoveEventSink(sink_.get());
        }
        sink_.reset();
        installed_ = false;
    }
#else
    bool Install() {
        return true;
    }

    void Uninstall() {}
#endif

    const RecapHotkeyBinding& owner_;
    ListenerState state_;
#if SKYRIM_LLM_HAS_RE_API
    std::unique_ptr<SkyrimInputEventSink> sink_;
    bool installed_{false};
#endif
};

RecapHotkeyBinding::~RecapHotkeyBinding() {
    UninstallSkyrimInputHook();
}

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

    listener_ = std::make_shared<Listener>(*this, ListenerState{
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

void RecapHotkeyBinding::UninstallSkyrimInputHook() {
    if (!listener_) {
        return;
    }
    listener_->Uninstall();
    registered_ = false;
}

bool RecapHotkeyBinding::MatchesRegisteredHotkey(std::uint32_t scan_code, bool pressed) const {
    return pressed && scan_code == keyboard_dx_scan_code_;
}

}  // namespace skyrim_llm::skse_host
