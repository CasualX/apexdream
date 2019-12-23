#include "state.hpp"
#include "process.hpp"
#include "data.hpp"

GameState::GameState()
	: signon_state(SignonState::None)
	, level_buffer{}
	, local_entity{}
	, max_clients{}
	, curtime{}
	, view_matrix{}
	, button_state{}
	, entities(new std::unique_ptr<BaseEntity>[NUM_ENT_ENTRIES]())
	, ent_info(new CEntInfo[NUM_ENT_ENTRIES]())
	, prev_info(new CEntInfo[NUM_ENT_ENTRIES]())
{}

void GameState::update(const GameProcess& process, const GameData& data) {
	process.read(process.r5apex_exe + data.signon_state, signon_state);
	process.read(process.r5apex_exe + data.level_name, level_buffer);
	process.read(process.r5apex_exe + data.local_entity, local_entity);

	uint64_t view_render_ptr, view_matrix_ptr;
	if (process.read(process.r5apex_exe + data.view_render, view_render_ptr)) {
		if (process.read(process.r5apex_exe + data.view_matrix, view_matrix_ptr)) {
			process.read(view_render_ptr, view_matrix);
		}
	}

	CGlobalVars global_vars;
	if (process.read(process.r5apex_exe + data.global_vars, global_vars)) {
		max_clients = global_vars.maxClients;
		curtime = global_vars.curtime;
	}

	process.read(process.r5apex_exe + data.input_system + data.input_button_state, button_state);

	if (process.read_array(process.r5apex_exe + data.entity_list, prev_info.get(), NUM_ENT_ENTRIES)) {
		std::swap(prev_info, ent_info);
		for (size_t i = 0; i < NUM_ENT_ENTRIES; i += 1) {
			if (prev_info[i].pEntity != ent_info[i].pEntity) {
				entities[i] = create_entity(process, i, ent_info[i].pEntity);
			}
			if (entities[i]) {
				entities[i]->update(process, data);
			}
		}
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
	// 48 8d 05 ? ? ? ?   mov rax, g_ClientClassForThisEntity
	// c3                 retn
	//
	// The question marks is x64 RIP-relative addressing mode displacement argument:
	// Calculate the final address by taking RIP after the instruction and add the displacement argument.
	uint32_t disp;
	if (!process.read(get_client_class + 3, disp)) {
		return false;
	}
	uint64_t client_class_ptr = get_client_class + disp + 7;
	// Get the ClientClass instance itself.
	ClientClass client_class;
	if (!process.read(client_class_ptr, client_class)) {
		return false;
	}
	// Finally grab some bytes to be interpreted as a nul terminated c-string.
	return process.read(client_class.pNetworkName, class_buffer);
}

std::unique_ptr<BaseEntity> GameState::create_entity(const GameProcess& process, uint32_t index, uint64_t entity_ptr) {
	// When the hack is running before the game has decrypted itself, these contain bad addresses
	if (entity_ptr == 0 || (entity_ptr & 0x7) != 0 || entity_ptr >= (1LL << 48)) {
		return {};
	}
	char class_buffer[128];
	if (!get_class_name(process, entity_ptr, class_buffer)) {
		return {};
	}
	if (!strcmp(class_buffer, "CPlayer")) {
		return std::make_unique<PlayerEntity>(entity_ptr);
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
