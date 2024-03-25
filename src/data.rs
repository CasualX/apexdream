#[derive(Default)]
pub struct GameData {
    pub version: String,

    pub time_date_stamp: u32,
    pub checksum: u32,

    pub global_vars: u32,

    pub entity_list: u32,
    pub local_entity_handle: u32,

    pub input: u32,
    pub input_selected_slot: u32,

    pub client_state: u32,
    pub client_signon_state: u32,
    pub client_level_name: u32,

    pub nst_weapon_names: u32,

    pub view_render: u32,
    pub view_matrix: u32,

    pub input_system: u32,
    pub input_button_state: u32,

    pub name_list: u32,

    pub highlight_settings: u32,

    pub network_var_table_ptr: u32,
    pub network_var_table_len: u32,

    pub thirdperson_override: u32,
    pub mouse_sensitivity: u32,
    pub fps_max: u32,
    pub mp_gamemode: u32,

    pub in_attack: u32,
    pub in_jump: u32,
    pub in_duck: u32,
    pub in_reload: u32,
    pub in_use: u32,
    pub in_zoom: u32,
    pub in_forward: u32,
    pub in_backward: u32,
    pub in_moveleft: u32,
    pub in_moveright: u32,

    pub entity_model_name: u32,
    pub entity_view_offset: u32,
    pub entity_flags: u32,
    pub entity_origin: u32,
    pub entity_shield_health: u32,
    pub entity_highlight: u32,
    pub entity_health: u32,
    pub entity_team_num: u32,
    pub entity_velocity: u32,
    pub entity_owner_entity: u32,
    pub entity_max_health: u32,
    pub entity_life_state: u32,

    pub animating_skin: u32,
    pub animating_bone_array: u32, // m_bSequenceFinished - 0x1C
    pub animating_studiohdr: u32,  // m_flModelScale + 0x1D0

    pub bcc_next_attack: u32,
    pub bcc_inventory: u32,
    pub bcc_selected_weapons: u32,
    pub bcc_last_visible_time: u32, // m_hudInfo_visibilityTestAlwaysPasses + 0x3

    pub player_zoom_state: u32,
    pub player_camera_data: u32,
    pub player_time_base: u32,
    pub player_server_angles: u32,
    pub player_view_angles: u32,
    pub player_weapon_punch: u32,
    pub player_consumables: u32,
    pub player_platform_uid: u32,
    pub player_bleedout_state: u32,
    pub player_movement_state: u32,
    pub player_observer_state: u32,
    pub player_third_person_shoulder_view: u32,
    pub player_script_net_data: u32,
    pub player_helmet_armor_type: u32,

    pub weaponx_weapon_owner: u32,
    pub weaponx_next_primary_attack: u32,
    pub weaponx_ammo_in_clip: u32,
    pub weaponx_player_data: u32,
    pub weaponx_zoom_fov: u32,
    pub weaponx_mod_bitfield: u32,
    pub weaponx_weapon_name_index: u32,
    pub weaponx_is_semi_auto: u32,
    pub weaponx_projectile_speed: u32,
    pub weaponx_charge_start_time: u32,
    pub weaponx_burst_fire: u32,

    pub vehicle_driver: u32,
    pub vehicle_velocity: u32,

    pub prop_survival: u32,
    pub projectile: u32,
    pub world_death_field: u32,
    pub waypoint_type: u32,

    pub mods_names: u32,
    pub mods_list: u32,
    pub mods_count: u32,
}

use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

impl Serialize for GameData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(Some(86))?; // number here is the amount of parameters in struct
        map.serialize_entry("version", &format!("{}", self.version))?;
        map.serialize_entry("time_date_stamp", &format!("0x{:x}", self.time_date_stamp))?;
        map.serialize_entry("checksum", &format!("0x{:x}", self.checksum))?;
        map.serialize_entry("global_vars", &format!("0x{:x}", self.global_vars))?;
        map.serialize_entry("entity_list", &format!("0x{:x}", self.entity_list))?;
        map.serialize_entry(
            "local_entity_handle",
            &format!("0x{:x}", self.local_entity_handle),
        )?;
        map.serialize_entry("input", &format!("0x{:x}", self.input))?;
        map.serialize_entry(
            "input_selected_slot",
            &format!("0x{:x}", self.input_selected_slot),
        )?;
        map.serialize_entry("client_state", &format!("0x{:x}", self.client_state))?;
        map.serialize_entry(
            "client_signon_state",
            &format!("0x{:x}", self.client_signon_state),
        )?;
        map.serialize_entry(
            "client_level_name",
            &format!("0x{:x}", self.client_level_name),
        )?;
        map.serialize_entry(
            "nst_weapon_names",
            &format!("0x{:x}", self.nst_weapon_names),
        )?;
        map.serialize_entry("view_render", &format!("0x{:x}", self.view_render))?;
        map.serialize_entry("view_matrix", &format!("0x{:x}", self.view_matrix))?;
        map.serialize_entry("input_system", &format!("0x{:x}", self.input_system))?;
        map.serialize_entry(
            "input_button_state",
            &format!("0x{:x}", self.input_button_state),
        )?;
        map.serialize_entry("name_list", &format!("0x{:x}", self.name_list))?;
        map.serialize_entry(
            "highlight_settings",
            &format!("0x{:x}", self.highlight_settings),
        )?;
        map.serialize_entry(
            "network_var_table_ptr",
            &format!("0x{:x}", self.network_var_table_ptr),
        )?;
        map.serialize_entry(
            "network_var_table_len",
            &format!("0x{:x}", self.network_var_table_len),
        )?;
        map.serialize_entry(
            "thirdperson_override",
            &format!("0x{:x}", self.thirdperson_override),
        )?;
        map.serialize_entry(
            "mouse_sensitivity",
            &format!("0x{:x}", self.mouse_sensitivity),
        )?;
        map.serialize_entry("fps_max", &format!("0x{:x}", self.fps_max))?;
        map.serialize_entry("mp_gamemode", &format!("0x{:x}", self.mp_gamemode))?;
        map.serialize_entry("in_attack", &format!("0x{:x}", self.in_attack))?;
        map.serialize_entry("in_jump", &format!("0x{:x}", self.in_jump))?;
        map.serialize_entry("in_duck", &format!("0x{:x}", self.in_duck))?;
        map.serialize_entry("in_reload", &format!("0x{:x}", self.in_reload))?;
        map.serialize_entry("in_use", &format!("0x{:x}", self.in_use))?;
        map.serialize_entry("in_zoom", &format!("0x{:x}", self.in_zoom))?;
        map.serialize_entry("in_forward", &format!("0x{:x}", self.in_forward))?;
        map.serialize_entry("in_backward", &format!("0x{:x}", self.in_backward))?;
        map.serialize_entry("in_moveleft", &format!("0x{:x}", self.in_moveleft))?;
        map.serialize_entry("in_moveright", &format!("0x{:x}", self.in_moveright))?;
        map.serialize_entry(
            "entity_model_name",
            &format!("0x{:x}", self.entity_model_name),
        )?;
        map.serialize_entry(
            "entity_view_offset",
            &format!("0x{:x}", self.entity_view_offset),
        )?;
        map.serialize_entry("entity_flags", &format!("0x{:x}", self.entity_flags))?;
        map.serialize_entry("entity_origin", &format!("0x{:x}", self.entity_origin))?;
        map.serialize_entry(
            "entity_shield_health",
            &format!("0x{:x}", self.entity_shield_health),
        )?;
        map.serialize_entry(
            "entity_highlight",
            &format!("0x{:x}", self.entity_highlight),
        )?;
        map.serialize_entry("entity_health", &format!("0x{:x}", self.entity_health))?;
        map.serialize_entry("entity_team_num", &format!("0x{:x}", self.entity_team_num))?;
        map.serialize_entry("entity_velocity", &format!("0x{:x}", self.entity_velocity))?;
        map.serialize_entry(
            "entity_owner_entity",
            &format!("0x{:x}", self.entity_owner_entity),
        )?;
        map.serialize_entry(
            "entity_max_health",
            &format!("0x{:x}", self.entity_max_health),
        )?;
        map.serialize_entry(
            "entity_life_state",
            &format!("0x{:x}", self.entity_life_state),
        )?;
        map.serialize_entry("animating_skin", &format!("0x{:x}", self.animating_skin))?;
        map.serialize_entry(
            "animating_bone_array",
            &format!("0x{:x}", self.animating_bone_array),
        )?;
        map.serialize_entry(
            "animating_studiohdr",
            &format!("0x{:x}", self.animating_studiohdr),
        )?;
        map.serialize_entry("bcc_next_attack", &format!("0x{:x}", self.bcc_next_attack))?;
        map.serialize_entry("bcc_inventory", &format!("0x{:x}", self.bcc_inventory))?;
        map.serialize_entry(
            "bcc_selected_weapons",
            &format!("0x{:x}", self.bcc_selected_weapons),
        )?;
        map.serialize_entry(
            "bcc_last_visible_time",
            &format!("0x{:x}", self.bcc_last_visible_time),
        )?;
        map.serialize_entry(
            "player_zoom_state",
            &format!("0x{:x}", self.player_zoom_state),
        )?;
        map.serialize_entry(
            "player_camera_data",
            &format!("0x{:x}", self.player_camera_data),
        )?;
        map.serialize_entry(
            "player_time_base",
            &format!("0x{:x}", self.player_time_base),
        )?;
        map.serialize_entry(
            "player_server_angles",
            &format!("0x{:x}", self.player_server_angles),
        )?;
        map.serialize_entry(
            "player_view_angles",
            &format!("0x{:x}", self.player_view_angles),
        )?;
        map.serialize_entry(
            "player_weapon_punch",
            &format!("0x{:x}", self.player_weapon_punch),
        )?;
        map.serialize_entry(
            "player_consumables",
            &format!("0x{:x}", self.player_consumables),
        )?;
        map.serialize_entry(
            "player_platform_uid",
            &format!("0x{:x}", self.player_platform_uid),
        )?;
        map.serialize_entry(
            "player_bleedout_state",
            &format!("0x{:x}", self.player_bleedout_state),
        )?;
        map.serialize_entry(
            "player_movement_state",
            &format!("0x{:x}", self.player_movement_state),
        )?;
        map.serialize_entry(
            "player_observer_state",
            &format!("0x{:x}", self.player_observer_state),
        )?;
        map.serialize_entry(
            "player_third_person_shoulder_view",
            &format!("0x{:x}", self.player_third_person_shoulder_view),
        )?;
        map.serialize_entry(
            "player_script_net_data",
            &format!("0x{:x}", self.player_script_net_data),
        )?;
        map.serialize_entry(
            "player_helmet_armor_type",
            &format!("0x{:x}", self.player_helmet_armor_type),
        )?;
        map.serialize_entry(
            "weaponx_weapon_owner",
            &format!("0x{:x}", self.weaponx_weapon_owner),
        )?;
        map.serialize_entry(
            "weaponx_next_primary_attack",
            &format!("0x{:x}", self.weaponx_next_primary_attack),
        )?;
        map.serialize_entry(
            "weaponx_ammo_in_clip",
            &format!("0x{:x}", self.weaponx_ammo_in_clip),
        )?;
        map.serialize_entry(
            "weaponx_player_data",
            &format!("0x{:x}", self.weaponx_player_data),
        )?;
        map.serialize_entry(
            "weaponx_zoom_fov",
            &format!("0x{:x}", self.weaponx_zoom_fov),
        )?;
        map.serialize_entry(
            "weaponx_mod_bitfield",
            &format!("0x{:x}", self.weaponx_mod_bitfield),
        )?;
        map.serialize_entry(
            "weaponx_weapon_name_index",
            &format!("0x{:x}", self.weaponx_weapon_name_index),
        )?;
        map.serialize_entry(
            "weaponx_is_semi_auto",
            &format!("0x{:x}", self.weaponx_is_semi_auto),
        )?;
        map.serialize_entry(
            "weaponx_projectile_speed",
            &format!("0x{:x}", self.weaponx_projectile_speed),
        )?;
        map.serialize_entry(
            "weaponx_charge_start_time",
            &format!("0x{:x}", self.weaponx_charge_start_time),
        )?;
        map.serialize_entry(
            "weaponx_burst_fire",
            &format!("0x{:x}", self.weaponx_burst_fire),
        )?;
        map.serialize_entry("vehicle_driver", &format!("0x{:x}", self.vehicle_driver))?;
        map.serialize_entry(
            "vehicle_velocity",
            &format!("0x{:x}", self.vehicle_velocity),
        )?;
        map.serialize_entry("prop_survival", &format!("0x{:x}", self.prop_survival))?;
        map.serialize_entry("projectile", &format!("0x{:x}", self.projectile))?;
        map.serialize_entry(
            "world_death_field",
            &format!("0x{:x}", self.world_death_field),
        )?;
        map.serialize_entry("waypoint_type", &format!("0x{:x}", self.waypoint_type))?;
        map.serialize_entry("mods_names", &format!("0x{:x}", self.mods_names))?;
        map.serialize_entry("mods_list", &format!("0x{:x}", self.mods_list))?;
        map.serialize_entry("mods_count", &format!("0x{:x}", self.mods_count))?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for GameData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GamedataVisitor;

        impl<'de> Visitor<'de> for GamedataVisitor {
            type Value = GameData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GameData, V::Error>
            where
                V: MapAccess<'de>,
            {
                // parse hex string into u32
                fn parse_hex(hex_str: &str) -> Result<u32, &'static str> {
                    u32::from_str_radix(&hex_str[2..], 16).map_err(|_| "Unable to parse hex string")
                }

                let mut gamedata = GameData::default();

                while let Some((key, value)) = map.next_entry::<String, String>()? {
                    match key.as_str() {
                        "version" => gamedata.version = value,
                        "time_date_stamp" => {
                            gamedata.time_date_stamp =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "checksum" => {
                            gamedata.checksum = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "global_vars" => {
                            gamedata.global_vars = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "entity_list" => {
                            gamedata.entity_list = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "local_entity_handle" => {
                            gamedata.local_entity_handle =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "input" => gamedata.input = parse_hex(&value).map_err(de::Error::custom)?,
                        "input_selected_slot" => {
                            gamedata.input_selected_slot =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "client_state" => {
                            gamedata.client_state = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "client_signon_state" => {
                            gamedata.client_signon_state =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "client_level_name" => {
                            gamedata.client_level_name =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "nst_weapon_names" => {
                            gamedata.nst_weapon_names =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "view_render" => {
                            gamedata.view_render = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "view_matrix" => {
                            gamedata.view_matrix = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "input_system" => {
                            gamedata.input_system = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "input_button_state" => {
                            gamedata.input_button_state =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "name_list" => {
                            gamedata.name_list = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "highlight_settings" => {
                            gamedata.highlight_settings =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "network_var_table_ptr" => {
                            gamedata.network_var_table_ptr =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "network_var_table_len" => {
                            gamedata.network_var_table_len =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "thirdperson_override" => {
                            gamedata.thirdperson_override =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "mouse_sensitivity" => {
                            gamedata.mouse_sensitivity =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "fps_max" => {
                            gamedata.fps_max = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "mp_gamemode" => {
                            gamedata.mp_gamemode = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "in_attack" => {
                            gamedata.in_attack = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "in_jump" => {
                            gamedata.in_jump = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "in_duck" => {
                            gamedata.in_duck = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "in_reload" => {
                            gamedata.in_reload = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "in_use" => {
                            gamedata.in_use = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "in_zoom" => {
                            gamedata.in_zoom = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "in_forward" => {
                            gamedata.in_forward = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "in_backward" => {
                            gamedata.in_backward = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "in_moveleft" => {
                            gamedata.in_moveleft = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "in_moveright" => {
                            gamedata.in_moveright = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "entity_model_name" => {
                            gamedata.entity_model_name =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "entity_view_offset" => {
                            gamedata.entity_view_offset =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "entity_flags" => {
                            gamedata.entity_flags = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "entity_origin" => {
                            gamedata.entity_origin = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "entity_shield_health" => {
                            gamedata.entity_shield_health =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "entity_highlight" => {
                            gamedata.entity_highlight =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "entity_health" => {
                            gamedata.entity_health = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "entity_team_num" => {
                            gamedata.entity_team_num =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "entity_velocity" => {
                            gamedata.entity_velocity =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "entity_owner_entity" => {
                            gamedata.entity_owner_entity =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "entity_max_health" => {
                            gamedata.entity_max_health =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "entity_life_state" => {
                            gamedata.entity_life_state =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "animating_skin" => {
                            gamedata.animating_skin =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "animating_bone_array" => {
                            gamedata.animating_bone_array =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "animating_studiohdr" => {
                            gamedata.animating_studiohdr =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "bcc_next_attack" => {
                            gamedata.bcc_next_attack =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "bcc_inventory" => {
                            gamedata.bcc_inventory = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "bcc_selected_weapons" => {
                            gamedata.bcc_selected_weapons =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "bcc_last_visible_time" => {
                            gamedata.bcc_last_visible_time =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "player_zoom_state" => {
                            gamedata.player_zoom_state =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "player_camera_data" => {
                            gamedata.player_camera_data =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "player_time_base" => {
                            gamedata.player_time_base =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "player_server_angles" => {
                            gamedata.player_server_angles =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "player_view_angles" => {
                            gamedata.player_view_angles =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "player_weapon_punch" => {
                            gamedata.player_weapon_punch =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "player_consumables" => {
                            gamedata.player_consumables =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "player_platform_uid" => {
                            gamedata.player_platform_uid =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "player_bleedout_state" => {
                            gamedata.player_bleedout_state =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "player_movement_state" => {
                            gamedata.player_movement_state =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "player_observer_state" => {
                            gamedata.player_observer_state =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "player_third_person_shoulder_view" => {
                            gamedata.player_third_person_shoulder_view =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "player_script_net_data" => {
                            gamedata.player_script_net_data =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "player_helmet_armor_type" => {
                            gamedata.player_helmet_armor_type =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "weaponx_weapon_owner" => {
                            gamedata.weaponx_weapon_owner =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "weaponx_next_primary_attack" => {
                            gamedata.weaponx_next_primary_attack =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "weaponx_ammo_in_clip" => {
                            gamedata.weaponx_ammo_in_clip =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "weaponx_player_data" => {
                            gamedata.weaponx_player_data =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "weaponx_zoom_fov" => {
                            gamedata.weaponx_zoom_fov =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "weaponx_mod_bitfield" => {
                            gamedata.weaponx_mod_bitfield =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "weaponx_weapon_name_index" => {
                            gamedata.weaponx_weapon_name_index =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "weaponx_is_semi_auto" => {
                            gamedata.weaponx_is_semi_auto =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "weaponx_projectile_speed" => {
                            gamedata.weaponx_projectile_speed =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "weaponx_charge_start_time" => {
                            gamedata.weaponx_charge_start_time =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "weaponx_burst_fire" => {
                            gamedata.weaponx_burst_fire =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "vehicle_driver" => {
                            gamedata.vehicle_driver =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "vehicle_velocity" => {
                            gamedata.vehicle_velocity =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "prop_survival" => {
                            gamedata.prop_survival = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "projectile" => {
                            gamedata.projectile = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "world_death_field" => {
                            gamedata.world_death_field =
                                parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "waypoint_type" => {
                            gamedata.waypoint_type = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "mods_names" => {
                            gamedata.mods_names = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "mods_list" => {
                            gamedata.mods_list = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        "mods_count" => {
                            gamedata.mods_count = parse_hex(&value).map_err(de::Error::custom)?
                        }
                        _ => eprintln!("Unknown field: {}", key),
                    }
                }

                Ok(gamedata)
            }
        }

        deserializer.deserialize_map(GamedataVisitor)
    }
}

impl GameData {
    pub fn load(&mut self, gd: &str, tds: u32) -> bool {
        let Ok(offset_json) = serde_json::from_str(gd) else {
            return false;
        };
        *self = offset_json;
        if self.time_date_stamp == tds {
            return true;
        } else {
            return false;
        }
    }
}
