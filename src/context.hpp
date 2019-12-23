#pragma once

#include <cstdint>

class GameProcess;
class GameData;
class GameState;

// The game context groups data together to make it convenient to pass around.
class GameContext {
public:
	GameContext(const GameProcess& process, const GameData& data, const GameState& state);

	// Checks if the given entity handle still points to the specified entity address.
	// Use this to reduce the chance of losing the race condition when writing to entity memory.
	bool entity_check(EHandle handle, uint64_t address) const;

public:
	const GameProcess& process;
	const GameData& data;
	const GameState& state;
	double time;
};
