use crate::*;
use std::collections::HashMap;
use std::fmt;

struct Config {
    enable: bool,
    weapon: u32,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            enable: false,
            weapon: 0,
        }
    }
}

#[derive(Default)]
pub struct Debugger {
    config: Config,
    timer: base::Timer,
}

impl Debugger {
    pub fn run(&mut self, api: &mut Api, ctx: &mut RunContext) {
        if self.timer.has_elapsed(ctx.state.time, 1.0 / 20.0)
            && (self.config.enable || ctx.state.is_firing_range())
        {
            self.state(api, ctx);
            self.world(api, ctx);
            self.player(api, ctx);
            self.loot(api, ctx);
            // self.deathbox(api, ctx);
            self.entities(api, ctx);
            self.resources(api, ctx);
            self.script_data(api, ctx);
            self.scriptnetdata(api, ctx);
            self.buttons(api, ctx);

            ctx.state.string_tables.debug(api);
        }
    }
    fn state(&mut self, api: &mut Api, ctx: &RunContext) {
        let state = ctx.state;
        api.visualize(
            s!("State"),
            xfmt! {
                <h3>"Client"</h3>
                <pre>
                    ("signon_state: "{state.client.signon_state}"\n")
                    ("level_name:   "{state.client.level_name}"\n")
                    ("game_mode:    "{state.gamemode():?}"\n")
                    ("local_entity: "{state.client.local_entity.signed_index()}"\n")
                    ("interval_per_tick: "{state.client.interval_per_tick}"\n")
                    ("view_render: "{state.client.view_render}"\n")
                    ("view_matrix_ptr: "{state.client.view_matrix_ptr}"\n")
                    ("view_matrix: "{state.client.view_matrix:#?}"\n")
                </pre>
                <h3>"InputSystem"</h3>
                <pre>
                    for i in (0..128) {
                        if (state.is_button_down(i)) {
                            {i}","
                        }
                    }
                </pre>
            },
        );
    }
    fn world(&mut self, api: &mut Api, ctx: &RunContext) {
        let state = ctx.state;
        let Some(local) = state.local_player() else {
            return;
        };
        let Some(world) = state.world_entity() else {
            return;
        };
        api.visualize(s!("World"), xfmt! {
			<pre>
				("is_active: "{world.death_field.is_active}"\n")
				("origin: "{world.death_field.origin:?}"\n")
				("radius: "{world.death_field.radius_start}" -> "{world.death_field.radius_end}"\n")
				("time: "{world.death_field.time_start}" -> "{world.death_field.time_end}"\n")
				("curtime: "{state.client.curtime}"\n")
				("distance: "{world.death_field.distance(state.client.curtime, local.camera_origin)}"\n")
			</pre>
		});
    }
    fn loot(&mut self, api: &mut Api, ctx: &RunContext) {
        let state = ctx.state;
        if let Some(local) = state.local_player() {
            let nearest = state
                .entities_as::<LootEntity>()
                .min_by_key(|loot| (sdk::dist(loot.origin, local.camera_origin) * 100.0) as i64);
            if let Some(loot) = nearest {
                api.visualize(
                    s!("PropSurvival"),
                    xfmt! {
                        <h3>"Nearest PropSurvival"</h3>
                        <pre>
                            ("index:  "{loot.index}"\n")
                            ("origin: "{loot.origin:?}"\n")
                            ("dist:   "{sdk::dist(loot.origin, local.camera_origin):.1}"\n")
                            "\n"
                            ("skin:       "{loot.skin}"\n")
                            ("skin_mod:   "{loot.skin_mod}"\n")
                            ("body:       "{loot.body}"\n")
                            ("camo_index: "{loot.camo_index}"\n")
                            ("model_name: "{loot.model_name.string}"\n")
                            "\n"
                            ("color: "{loot.color:?}"\n")
                            ("bits:  "{loot.bits:?}"\n")
                            "\n"
                            ("ammo_in_clip:      "{loot.ammo_in_clip}"\n")
                            ("custom_script_int: "{loot.custom_script_int}"\n")
                            ("survival_property: "{loot.survival_property}"\n")
                            ("weapon_index:      "{loot.weapon_name_index}"\n")
                            ("mod_bitfield:      "{loot.mod_bitfield:#010x}"\n")
                            |f| print_weapon_mods(f, state, loot.weapon_name, loot.mod_bitfield)?;
                            "\n"
                            ("weapon_name:  "{state.weapon_string(loot.weapon_name_index):?}"\n")
                            ("known_item:   "{loot.known_item}"\n")
                        </pre>
                    },
                );
            }
        }
    }
    fn player(&mut self, api: &mut Api, ctx: &RunContext) {
        let state = ctx.state;
        if let Some(player) = state.local_player() {
            player.studio.visualize(api, s!("PlayerStudio"));

            api.visualize(s!("Player"), xfmt! {
				(<div style="display: grid; grid-auto-flow: column; grid-auto-columns: 600px;">
				<div>
					<h3>"Player"</h3>
					<pre>)
						("origin:    "{player.origin:?}"\n")
						("angles:    "{player.angles:?}"\n")
						("velocity:  "{player.velocity:?}"\n")
						("view_offset:"{player.view_offset:?}"\n")
						("accel:     "{player.accel:?}"\n")
						("accel_len: "{player.accel_angle()}"\n")
						"\n"
						("health:   "{player.health}"/"{player.max_health}"\n")
						("shields:  "{player.shields}"/"{player.max_shields}"\n")
						"\n"
						("life_state:     "{player.life_state}"\n")
						("bleedout_state: "{player.bleedout_state}"\n")
						("flags:          "{player.flags;#x}"\n")
						"\n"
						("camera origin: "{player.camera_origin:?}"\n")
						("camera angles: "{player.camera_angles:?}"\n")
						("server angles: "{player.server_angles:?}"\n")
						("view angles:   "{player.view_angles:?}"\n")
						("breath angles: "{player.breath_angles:?}"\n")
						("punch angle:   "{player.breath_angles[0] - player.view_angles[0]}"\n")
						("weapon_punch:  "{player.weapon_punch:?}"\n")
						"\n"
						("ground entity: "{player.ground_entity:?}"\n")
						"\n"
						("active_weapon:  "{player.active_weapon:?}"\n")
						("primary_weapon: "{player.primary_weapon:?}"\n")
						("selected_slot:  "{player.selected_slot}"\n")
						("weapons: "{player.weapons;#?}"\n")
						"\n"
						for weapon in (state.entities_as::<WeaponXEntity>().filter(|w| w.weapon_owner == player.get_info().handle)) {
							{weapon.index}": "{state.weapon_string(weapon.weapon_name_index):?}"\n"
						}
						"\n"
						("consumables:\n")
						for item in &player.consumables {
							if (item.count != 0) {
								"  "{state.known_item(item.item as i32)}": "{item.count}"\n"
							}
						}
						"\n"
						("observer_mode:   "{player.observer_mode}"\n")
						("observer_target: "{player.observer_target:?}"\n")
					</pre>
				</div>
				let weapon = if self.config.weapon > 0 {
					state.entity_as::<WeaponXEntity>(sdk::EHandle::from(self.config.weapon))
				}
				else {
					player.active_weapon(state)
				};
				if let Some(weapon) = weapon {
					(<div>
						<h3>if self.config.weapon > 0 { "WeaponX" } else { "Active WeaponX" }</h3>
						<pre>)
							("index: "{weapon.index}"\n")
							"\n"
							("mod_bitfield: "{weapon.mod_bitfield;#010x}"\n")
							|f| print_weapon_mods(f, state, weapon.weapon_name, weapon.mod_bitfield)?;
							"\n"
							("time_base:               "{player.time_base}"\n")
							("last_primary_attack_time:"{weapon.last_primary_attack_time}"\n")
							("next_ready_time:         "{weapon.next_ready_time}"\n")
							("time to next attack:     "{weapon.next_ready_time - player.time_base}"\n")
							("next_primary_attack_time:"{weapon.next_primary_attack_time}"\n")
							("attack_time_this_frame:  "{weapon.attack_time_this_frame}"\n")
							"\n"
							("ammo_in_clip:      "{weapon.ammo_in_clip}"\n")
							("ammo_in_stockpile: "{weapon.ammo_in_stockpile}"\n")
							("ammo in backpack:  "{player.get_consumable_count(state, weapon.ammo_item(state))}"\n")
							("lifetime_shots:    "{weapon.lifetime_shots}"\n")
							("weap_state:        "{weapon.weap_state.0}"\n")
							("discarded:         "{weapon.discarded}"\n")
							("in_reload:         "{weapon.in_reload}"\n")
							"\n"
							("burst_fire_count: "{weapon.burst_fire_count}"\n")
							("burst_fire_index: "{weapon.burst_fire_index}"\n")
							("shot_index:       "{weapon.shot_index}"\n")
							("shot_count:       "{weapon.shot_count}"\n")
							"\n"
							("cur_zoom_fov:      "{weapon.cur_zoom_fov}"\n")
							("target_zoom_fov:   "{weapon.target_zoom_fov}"\n")
							"\n"
							("weapon_name_index: "{weapon.weapon_name_index}"\n")
							("weapon_name:       "{state.weapon_string(weapon.weapon_name_index):?}"\n")
							"\n"
							("is_semi_auto:      "{weapon.is_semi_auto}"\n")
							("projectile_speed:  "{weapon.projectile_speed}"\n")
							("projectile_scale:  "{weapon.projectile_scale}"\n")
						</pre>
					</div>
				}
				</div>
			});
        }
    }
    fn entities(&mut self, api: &mut Api, ctx: &RunContext) {
        let state = ctx.state;
        let mut stats = HashMap::<String, usize>::new();
        let mut name_buf = [0u8; 32];
        let mut count = 0usize;
        for entity in state.entities() {
            let type_name = entity.as_ref().get_type_name(&mut name_buf);
            count += 1;
            if let Some(e) = stats.get_mut(type_name) {
                *e += 1;
            } else {
                stats.insert(String::from(type_name), 1);
            }
        }
        let mut keys = stats.keys().into_iter().collect::<Vec<_>>();
        keys.sort_unstable();

        api.visualize(
            s!("Entities"),
            xfmt! {
                <pre>
                    "Updates: "{state.entity_list.updates}"\n"
                    "Entities: "{count}"\n"
                    for &key in (&keys) {
                        {key}": "{stats.get(key).unwrap_or(&0)}"\n"
                    }
                </pre>
            },
        );
    }
    fn resources(&mut self, api: &mut Api, ctx: &RunContext) {
        let state = ctx.state;
        api.visualize(s!("Resources"), xfmt! {
			<pre><table>
				<tr>
					<th>"Name"</th>
					<th>"Equals"</th>
					<th>"PlatformUID"</th>
					<th>"TeamNum"</th>
					<th>"LifeState"</th>
					<th>"Observing"</th>
				</tr>
				for i in 0..sdk::MAX_PLAYERS {
					let handle = sdk::EHandle::from(i as u32);
					let player = state.entity_as::<PlayerEntity>(handle);
					if (player.is_some()) {
						<tr>
							<td>{state.get_player_name(handle).unwrap_or("_")}</td>
							<td>if (state.get_name1(i - 1) == state.get_name2(i - 1)) { "-" } else { "XXX" }</td>
							if let Some(player) = (player) {
								<td>{player.platform_uid}</td>
								<td>{player.team_num}"("{player.team_member_index}")"</td>
								<td>
									if player.is_downed() { "Downed" }
									else if player.is_alive() { "Alive" }
									else if player.observer_target.is_valid() { "Observing" }
									else { "Dead" }
								</td>
								<td>if let Some(name) = (state.get_player_name(player.observer_target)) { {name} }</td>
							}
						</tr>
					}
				}
			</table></pre>
		});
    }
    fn script_data(&mut self, api: &mut Api, ctx: &RunContext) {
        let state = &ctx.state.script_data.entries[..];
        api.visualize(
            s!("ScriptData"),
            xfmt! {
                <pre><table>
                <tr>
                    <td>"Index"</td>
                    <td>"Name"</td>
                    <td>"Cat"</td>
                    <td>"Type"</td>
                    <td>"Offset"</td>
                </tr>
                for (i, e) in (state.iter().enumerate()) {
                    if (e.name_hash != 0) {
                        <tr>
                        <td>"$"{i;#x}</td>
                        <td>{e.name_hash;#x?}</td>
                        <td>{e.category}</td>
                        <td>{e.value_type}</td>
                        <td>{e.value_index}</td>
                        <td>{e.unk0x14}</td>
                        <td>{e.unk0x18}</td>
                        <td>{e.unk0x1c}</td>
                        <td>{e.unk0x20;#x}</td>
                        <td>{e.unk0x24}</td>
                        <td>{e.unk0x28}</td>
                        <td>{e.unk0x30}</td>
                        <td>{e.unk0x34}</td>
                        </tr>
                    }
                }
                </table></pre>
            },
        );
    }
    fn scriptnetdata(&self, api: &mut Api, ctx: &RunContext) {
        let state = ctx.state;
        let Some(local) = state.local_player() else {
            return;
        };
        let Some(global) = state.entity_as::<ScriptNetDataEntity>(local.script_net_data_global)
        else {
            return;
        };
        let Some(exclusive) =
            state.entity_as::<ScriptNetDataEntity>(local.script_net_data_exclusive)
        else {
            return;
        };
        api.visualize(
            s!("ScriptNetData"),
            xfmt! {
                <h1>"Global"</h1>
                <pre>
                    ("Bools: "{global.bools():?}"\n")
                    ("Shorts: "{global.shorts():?}"\n")
                    ("Ints: "{global.ints():?}"\n")
                    ("Floats: "{global.floats():?}"\n")
                    ("Ents: "{global.ents():?}"\n")
                </pre>
                <h1>"Exclusive"</h1>
                <pre>
                    ("Bools: "{exclusive.bools():?}"\n")
                    ("Shorts: "{exclusive.shorts():?}"\n")
                    ("Ints: "{exclusive.ints():?}"\n")
                    ("Floats: "{exclusive.floats():?}"\n")
                    ("Ents: "{exclusive.ents():?}"\n")
                </pre>
            },
        );
    }
    fn buttons(&mut self, api: &mut Api, ctx: &RunContext) {
        let ref buttons = ctx.state.buttons;
        api.visualize(
            s!("Buttons"),
            xfmt! {
                <pre>
                    "in_attack:    "{buttons.in_attack.state}"\n"
                    "in_jump:      "{buttons.in_jump.state}"\n"
                    "in_reload:    "{buttons.in_reload.state}"\n"
                    "in_use:       "{buttons.in_use.state}"\n"
                    "in_zoom:      "{buttons.in_zoom.state}"\n"
                    "in_forward:   "{buttons.in_forward.state}"\n"
                    "in_backward:  "{buttons.in_backward.state}"\n"
                    "in_moveleft:  "{buttons.in_moveleft.state}"\n"
                    "in_moveright: "{buttons.in_moveright.state}"\n"
                </pre>
            },
        );
    }
}

fn print_weapon_mods(
    f: &mut fmt::Formatter,
    state: &GameState,
    weapon_name: sdk::WeaponName,
    mod_bitfield: u32,
) -> fmt::Result {
    fmtools::write!(
        f,
        if let Some(mods) = state.get_mods(weapon_name) {
            for (i, mod_name) in (mods.iter().enumerate()) {
                if (mod_bitfield & (1 << i) != 0) {
                    {
                        mod_name
                    }
                    "\n"
                }
            }
        }
    )
}
