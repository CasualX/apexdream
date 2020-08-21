#include "context.hpp"
#include "process.hpp"
#include "data.hpp"

GameContext::GameContext(const GameProcess& process, const GameState& state)
	: process(process), state(state), time(get_time()) {}
void GameContext::pre() {
	attack.update(process, data::IN_ATTACK);
	jump.update(process, data::IN_JUMP);
}
void GameContext::post() {
	attack.post(process, data::IN_ATTACK);
	jump.post(process, data::IN_JUMP);
}

bool GameContext::entity_check(EHandle handle, uint64_t address) const {
	if (!handle.is_valid()) {
		return false;
	}
	const uint32_t offset = static_cast<uint32_t>(handle.index() * sizeof(CEntInfo));
	uint64_t check;
	return process.read(process.r5apex_exe + data::ENTITY_LIST + offset, check) && address == check;
}

void InState::update(const GameProcess& process, uint32_t address) {
	process.read(process.r5apex_exe + address, button);
	state = (button.state & 1) != 0;
}
void InState::post(const GameProcess& process, uint32_t address) {
	// If active get the most recent state of the button
	if (force && process.read(process.r5apex_exe + address, button)) {
		// Get the desired state of the button
		int state;
		if (press && !release) {
			state = 5;
		}
		else if (!press && release) {
			state = 4;
		}
		else {
			state = button.down[0] == 0 && button.down[1] == 0 ? 4 : 5;
		}
		// Gently tell the game to that nobody will be harmed if they just do as told
		if (button.state != state) {
			process.write(process.r5apex_exe + address + 8, state);
		}
	}
}
