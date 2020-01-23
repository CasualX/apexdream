#include "data.hpp"
#include "process.hpp"
#include "state.hpp"
#include "cheats.hpp"
#include "context.hpp"

void apexbot(uint32_t pid) {
	// Attach to the process
	const GameProcess process{pid};
	if (process.r5apex_exe == 0) {
		printf("apex(%d) Access denied, did you implement EAC bypass?\n", pid);
		return;
	}
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
			ctx.pre();
			cheats.run(ctx);
			ctx.post();
		}
	}
	else {
		printf("apex(%d) Gamedata mismatch! Please update the offsets.\n", pid);
	}
}

int main(int argc, char* argv[]) {
	(void)argc;
	(void)argv;
	init_time();
	// Track the last attached process id to prevent reattaching accidentally
	uint32_t last_process_id = ~0;
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
				apexbot(entry.id);
				last_process_id = entry.id;
				seen_last_process_id = true;
				break;
			}
		}
		// Clear the last process id if it hasn't been seen
		if (!seen_last_process_id) {
			last_process_id = ~0;
		}
		// Wait before looking again
		sleep(100);
	}
	return 0;
}
