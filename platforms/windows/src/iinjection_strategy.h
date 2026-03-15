#pragma once
// IInjectionStrategy — Open/Closed + Dependency Inversion for text injection.
// New injection strategies (e.g. AX direct, clipboard) are added by implementing
// this interface without touching TextInjector or KeyboardHook.

#include <cstdint>
#include "app_compat.h"

namespace goxviet {

class IInjectionStrategy {
public:
    virtual ~IInjectionStrategy() = default;

    // Inject `backspaces` delete events, then the UTF-8 replacement text.
    virtual void Inject(int backspaces, const char* utf8Text,
                        const InjectionTiming& timing) = 0;
};

}  // namespace goxviet
