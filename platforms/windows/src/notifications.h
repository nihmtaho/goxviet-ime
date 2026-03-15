#pragma once
#include <functional>
#include <unordered_map>
#include <vector>
#include <mutex>

// Typed event bus — equivalent to TypedNotifications.swift on macOS.
// Provides loose coupling between components (Dependency Inversion Principle).

namespace goxviet {

enum class AppEvent {
    EnabledChanged,
    InputMethodChanged,
    ToneStyleChanged,
    SmartModeChanged,
    InstantRestoreChanged,
    EscRestoreChanged,
    FreeToneChanged,
    ShiftBackspaceChanged,
    ShortcutsChanged,
    OutputEncodingChanged,
    PerAppModesChanged,
    InputLanguageChanged,   // non-Latin keyboard detected
};

using EventHandler = std::function<void(AppEvent)>;

class EventBus {
public:
    static EventBus& Instance();

    // Subscribe with an opaque owner key; replaces any existing subscription from same owner.
    void Subscribe(AppEvent event, void* owner, EventHandler handler);

    // Remove all subscriptions for this owner.
    void Unsubscribe(void* owner);

    // Fire event on calling thread.
    void Post(AppEvent event);

private:
    EventBus() = default;
    EventBus(const EventBus&) = delete;
    EventBus& operator=(const EventBus&) = delete;

    struct Subscription { void* owner; EventHandler handler; };
    std::unordered_map<int, std::vector<Subscription>> subs_;
    std::mutex mutex_;
};

}  // namespace goxviet
