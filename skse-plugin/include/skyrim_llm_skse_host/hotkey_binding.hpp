#pragma once

#include <cstdint>
#include <functional>

namespace skyrim_llm::skse_host {

class RecapHotkeyBinding {
public:
    static constexpr std::uint32_t kDefaultKeyboardDxScanCode = 0x42;  // F8

    void SetHandler(std::function<void()> handler);
    void SetKeyboardDxScanCode(std::uint32_t scan_code);
    bool RegisterDefaultHotkey();
    bool IsRegistered() const;
    void DispatchForTesting() const;

private:
    bool InstallSkyrimInputHook();
    bool MatchesRegisteredHotkey(std::uint32_t scan_code, bool pressed) const;

    std::function<void()> handler_;
    std::uint32_t keyboard_dx_scan_code_{kDefaultKeyboardDxScanCode};
    bool registered_{false};
};

}  // namespace skyrim_llm::skse_host
