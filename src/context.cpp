#include "context.hpp"
#include "process.hpp"
#include "data.hpp"

GameContext::GameContext(const GameProcess& process, const GameData& data, const GameState& state) : process(process), data(data), state(state), time(float_time()) {}

bool GameContext::entity_check(EHandle handle, uint64_t address) const {
	if (!handle.is_valid()) {
		return false;
	}
	const uint32_t offset = handle.index() * sizeof(CEntInfo);
	uint64_t check;
	return process.read(process.r5apex_exe + data.entity_list + offset, check) && address == check;
}
