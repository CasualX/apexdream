/*!
# The Apex Legends Dream

Enjoy! - Casual_Hacker
*/

#![recursion_limit = "1024"]

use bitset_core::*;
use fmtools::fmt as f;
use format_xml::xfmt;
use obfstr::obfstr as s;

#[macro_use]
mod base;
use self::base::hash;

mod interface;
pub use self::interface::Interface;

mod api;
mod cheats;
mod data;
mod process;
mod sdk;
mod state;

use self::api::*;
use self::cheats::*;
use self::data::GameData;
use self::process::GameProcess;
use self::state::entities::*;
use self::state::{GameState, UpdateContext};

const STRING_POOL: &str = concat!(
	"tickTICKmrvnMRVNProwlerDUMMIEDUMMYBaseNPCLOOT^.^",
	"MastiffPKEVA8R301SentinelFlatlineWingmanCARKraberScoutVoltBocekRampageHemlokRE45NemesisHemlokC.A.R99Havoc",
	"OObinBINGASPADTOTEMSEERPYLONAnim");

/// Cheat instance.
#[derive(Default)]
pub struct Instance {
    process: GameProcess,
    data: GameData,
    state: GameState,
    cheats: CheatManager,
    tickcount: u32,
    pool: base::StringPool,
}

impl Instance {
    /// Try to attach with specified gamedata.
    ///
    /// Returns `false` on failure, details are logged.
    pub fn attach(&mut self, api: &mut dyn Interface, gd: &str) -> bool {
        let api = Api(api);

        if !self.process.attach(api) {
            api.log(f!("Attach error!"));
            return false;
        }

        if !self.data.load(gd, self.process.time_date_stamp) {
            api.log(f!("Gamedata mismatch!"));
            return false;
        }

        api.log(f!("Attached!"));
        return true;
    }

    /// Ticks the instance.
    ///
    /// Must call [`attach`](Self::attach) before trying to tick the instance.
    /// If attach is not successful, this method does nothing.
    pub fn tick(&mut self, api: &mut dyn Interface) {
        let api = Api(api);
        let time = api.get_time();

        // Dump the game binary
        self.process.read_pages(api, time);

        // Check if process is valid
        if !self.process.is_valid(api) {
            return;
        }
        // Check if gamedata is valid for this process
        if self.process.time_date_stamp != self.data.time_date_stamp {
            return;
        }
        // Wait for the game process to deobfuscate itself
        if !self.process.is_ready(api, time) {
            return;
        }

        // Update our state of the game world
        {
            let local_entity = self.state.client.local_entity;
            self.state.time = time;
            let ref mut ctx = UpdateContext {
                process: &self.process,
                data: &self.data,
                time,
                connected: false,
                tickcount: self.tickcount,
                local_entity,
                full_bones: self.cheats.full_bones,
            };
            self.state.update(api, ctx);
        }

        // Load the current weapon settings
        let config_section = self.state.get_config_section();

        // Run the cheat modules
        {
            s! { let strings = STRING_POOL; }
            let mut ctx =
                RunContext::new(&self.process, &self.data, &self.state, &strings, &self.pool);
            self.cheats.run(api, &mut ctx);
            ctx.post(api);
        }

        self.tickcount = self.tickcount.wrapping_add(1);
        self.pool.clear();
    }

    /// Loads a config string.
    pub fn load_config(&mut self, api: &mut dyn Interface) {
        let api = Api(api);
    }
}
