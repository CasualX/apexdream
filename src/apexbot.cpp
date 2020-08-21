#include "data.hpp"
#include "process.hpp"
#include "state.hpp"
#include "cheats.hpp"
#include "context.hpp"
#include "config.hpp"

void apexbot(uint32_t pid) {
	// Attach to the process
	const GameProcess process{pid};
	if (process.r5apex_exe == 0) {
		printf("apex(%u) Access denied, did you implement EAC bypass?\n", pid);
		return;
	}
	// Initialize the game's offsets data
	// Check if the offsets are valid for this game version
	if (process.check_version(data::TIME_DATE_STAMP, data::CHECKSUM)) {
		GameState state{};
		CheatManager cheats{};
		Config config{};
		// The heart of the cheat is simple, repeat until the process dies
		while (process.heartbeat()) {
			// Update our copy of the game state
			state.update(process);
			// Reload the weapon config
			config.run(state, cheats);
			// Run the cheat modules
			GameContext ctx{process, state};
			ctx.pre();
			cheats.run(ctx);
			ctx.post();
		}
	}
}

int main(int argc, char* argv[]) {
	(void)argc;
	(void)argv;
	init_time();
	// Track the last attached process id to prevent reattaching accidentally
	uint32_t last_process_id = ~0U;
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
