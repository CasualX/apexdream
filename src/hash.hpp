#pragma once

#include <cstdint>

constexpr uint32_t hash(const char* s) {
	uint32_t hash = 5381;
	for (size_t i = 0; s[i] != '\0'; i += 1) {
		hash = hash * 33 + static_cast<uint8_t>(s[i]);
	}
	return hash;
}
