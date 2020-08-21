#include "state.hpp"
#include "process.hpp"
#include "data.hpp"

GameState::GameState()
	: signon_state(SignonState::None)
	, connected{}
	, level_buffer{}
	, local_entity{}
	, max_clients{}
	, curtime{}
	, VIEW_MATRIX{}
	, button_state{}
	, entities(new std::unique_ptr<BaseEntity>[NUM_ENT_ENTRIES]())
	, ent_info(new CEntInfo[NUM_ENT_ENTRIES]())
	, prev_info(new CEntInfo[NUM_ENT_ENTRIES]())
	, weapon_names{}
{}

void load_string_table(std::vector<std::string>& names, const GameProcess& process, uint32_t offset) {
	uint64_t table_ptr;
	if (!process.read(process.r5apex_exe + offset, table_ptr)) {
		return;
	}
	CNetStringTable table;
	if (!process.read(table_ptr, table)) {
		return;
	}
	CNetStringDict dict;
	if (!process.read(table.items, dict)) {
		return;
	}
	names.resize(dict.used);
	char buffer[64];
	CNetStringTableItem item;
	for (size_t i = 0; i < dict.used; i += 1) {
		names[i].clear();
		if (process.read(dict.elements + i * 72, item) && process.read(item.string, buffer)) {
			names[i].append(buffer);
		}
	}
}

void GameState::update(const GameProcess& process) {
	auto old_signon_state = signon_state;

	process.read(process.r5apex_exe + data::CLIENT_STATE + data::CLIENT_SIGNON_STATE, signon_state);
	process.read(process.r5apex_exe + data::CLIENT_STATE + data::CLIENT_LEVEL_NAME, level_buffer);
	process.read(process.r5apex_exe + data::LOCAL_ENTITY, local_entity);

	connected = old_signon_state != SignonState::Full && signon_state == SignonState::Full;

	uint64_t view_render_ptr, view_matrix_ptr;
	if (process.read(process.r5apex_exe + data::VIEW_RENDER, view_render_ptr)) {
		if (process.read(view_render_ptr + data::VIEW_MATRIX, view_matrix_ptr)) {
			process.read(view_matrix_ptr, VIEW_MATRIX);
		}
	}

	CGlobalVars global_vars;
	if (process.read(process.r5apex_exe + data::GLOBAL_VARS, global_vars)) {
		max_clients = global_vars.maxClients;
		curtime = global_vars.curtime;
	}

	process.read(process.r5apex_exe + data::INPUT_SYSTEM + data::INPUT_BUTTON_STATE, button_state);

	if (process.read_array(process.r5apex_exe + data::ENTITY_LIST, prev_info.get(), NUM_ENT_ENTRIES)) {
		std::swap(prev_info, ent_info);
		for (uint32_t i = 0; i < NUM_ENT_ENTRIES; i += 1) {
			if (prev_info[i].pEntity != ent_info[i].pEntity) {
				entities[i] = create_entity(process, i, ent_info[i].pEntity);
			}
			if (entities[i]) {
				entities[i]->update(process);
			}
		}
	}

	if (connected) {
		load_string_table(weapon_names, process, data::NST_WEAPON_NAMES);
	}
}

static bool get_class_name(const GameProcess& process, uint64_t entity_ptr, char (&class_buffer)[128]) {
	// Internally this simulate the following code:
	// return pEntity->GetClientClass()->pNetworkName;
	// Get the IClientNetworkable vtable.
	uint64_t client_networkable_vtable;
	if (!process.read(entity_ptr + 8 * 3, client_networkable_vtable)) {
		return false;
	}
	// Get the GetClientClass virtual function pointer.
	uint64_t get_client_class;
	if (!process.read(client_networkable_vtable + 8 * 3, get_client_class)) {
		return false;
	}
	// It's a pointer to the following asm:
	//
	// 48 8d 05 ? ? ? ?   lea rax, g_ClientClassForThisEntity
	// c3                 retn
	//
	// The question marks is x64 RIP-relative addressing mode displacement argument:
	// Calculate the final address by taking RIP after the instruction and add the displacement argument.
	uint32_t disp;
	if (!process.read(get_client_class + 3, disp)) {
		return false;
	}
	const uint64_t client_class_ptr = get_client_class + disp + 7;
	// Get the ClientClass instance itself.
	ClientClass client_class;
	if (!process.read(client_class_ptr, client_class)) {
		return false;
	}
	// Finally grab some bytes to be interpreted as a nul terminated c-string.
	return process.read(client_class.pNetworkName, class_buffer);
}

std::unique_ptr<BaseEntity> GameState::create_entity(const GameProcess& process, uint32_t index, uint64_t entity_ptr) {
	if (entity_ptr == 0) {
		return {};
	}
	// When the hack is running before the game has decrypted itself, these contain bad addresses
	if ((entity_ptr & 0x7) != 0 || entity_ptr >= (1ULL << 48)) {
		return {};
	}
	char class_buffer[128];
	if (!get_class_name(process, entity_ptr, class_buffer)) {
		return {};
	}
	if (!strcmp(class_buffer, "CPlayer")) {
		return std::make_unique<PlayerEntity>(entity_ptr);
	}
	if (!strcmp(class_buffer, "CAI_BaseNPC")) {
		return std::make_unique<BaseNPCEntity>(entity_ptr);
	}
	if (!strcmp(class_buffer, "CPropSurvival")) {
		return std::make_unique<PropSurvivalEntity>(entity_ptr);
	}
	if (!strcmp(class_buffer, "CWeaponX")) {
		return std::make_unique<WeaponXEntity>(entity_ptr);
	}
	if (!strcmp(class_buffer, "CPlayerResource")) {
		resources_entity = EHandle{index};
		return std::make_unique<PlayerResourceEntity>(entity_ptr);
	}
	if (!strcmp(class_buffer, "CWorld")) {
		return std::make_unique<WorldEntity>(entity_ptr);
	}
	// Uninteresting entity type, uncomment this print statement to discover entity types
	// printf("Uninteresting: %s\n", class_buffer);
	return {};
}
