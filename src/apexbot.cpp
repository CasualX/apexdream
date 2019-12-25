#include "items.hpp"
#include "data.hpp"
#include "sdk.hpp"
#include "entities.hpp"
#include "process.hpp"
#include "state.hpp"
#include "cheats.hpp"
#include "context.hpp"

void apex_legends(uint32_t pid) {
	// Attach to the process
	const GameProcess process{pid};
	// Initialize the game's offsets data
	static const GameData data{};
	// Check if the offsets are valid for this game version
	if (process.check_version(data.time_date_stamp, data.checksum)) {
		GameState state{};
		CheatManager cheats{};
		// The heart of the cheat is simple, repeat until the process dies
		while (process.heartbeat()) {
			// Update our copy of the game state
			state.update(process, data);
			// Run the cheat modules
			GameContext ctx{process, data, state};
			cheats.run(ctx);
		}
	}
	else {
		printf("apex(%d) Gamedata mismatch!\n", pid);
	}
}

int main(int argc, char* argv[]) {
	(void)argc;
	(void)argv;
	// Track the last attached process id to prevent reattaching accidentally
	uint32_t last_process_id = 0;
	while (true) {
		bool seen_last_process_id = false;
		ProcessEntry entry;
		for (ProcessEnumerator processes{}; processes.next(entry); ) {
			// Ignore the last process id until it has gone away
			if (entry.id == last_process_id) {
				seen_last_process_id = true;
				continue;
			}
			// Find the Apex Legends process
			if (!wcscmp(entry.name, L"r5apex.exe")) {
				apex_legends(entry.id);
				last_process_id = entry.id;
				seen_last_process_id = true;
				break;
			}
		}
		// Clear the last process id if it hasn't been seen
		if (!seen_last_process_id) {
			last_process_id = 0;
		}
		// Wait before looking again
		sleep(100);
	}
	return 0;
}
