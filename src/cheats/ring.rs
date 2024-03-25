use crate::*;

const TICK_TIME: f32 = 1.5;

const WIDTH: f32 = 200.0;
const HEIGHT: f32 = 35.0;

struct Config {
    enable: bool,
}
impl Default for Config {
    fn default() -> Self {
        Config { enable: true }
    }
}

#[derive(Default)]
pub struct RingDamage {
    config: Config,
    health: i32,
    ring_damage: f32,
    tick_time: f64,
}

impl RingDamage {
    pub fn render(&mut self, api: &mut Api, ctx: &RunContext) {
        if !self.config.enable {
            return;
        }

        let state = ctx.state;
        let Some(local) = state.local_player() else {
            return;
        };
        let Some(world) = state.world_entity() else {
            return;
        };
        if !world.death_field.is_active {
            self.health = 0;
            self.ring_damage = 0.0;
            return;
        }

        // Must be 200 units near the edge of the ring
        let curtime = state.client.curtime;
        let distance = world.death_field.distance(curtime, local.view_origin);
        if !(distance > -128.0) {
            let fract = f32::fract(curtime / TICK_TIME);
            self.tick_time = state.time - (fract * TICK_TIME) as f64;
            return;
        }

        // Dynamically update ring damage from health
        if self.health != local.health {
            // Estimate when we should be taking damage from zone
            if distance > 0.0 && local.health < self.health {
                self.ring_damage = (self.health - local.health) as f32;
                self.tick_time = state.time;
            }
            self.health = local.health;
        }

        // How far along we are to the next time zone will damage
        let fract = f32::fract((state.time - self.tick_time) as f32 / TICK_TIME);

        let xcenter = (ctx.screen[0] >> 1) as f32;
        let ypos = ctx.screen[1] as f32 - 250.0;

        api.r_rect(
            /*x:*/ xcenter - WIDTH * 0.5,
            /*y:*/ ypos,
            /*width:*/ WIDTH * (1.0 - fract),
            /*height:*/ HEIGHT,
            /*fill:*/ vgc::sRGBA!(White, 0x80),
            /*stroke:*/ vgc::sRGBA::TRANSPARENT,
        );
        api.r_rect(
            /*x:*/ xcenter - WIDTH * 0.5,
            /*y:*/ ypos,
            /*width:*/ WIDTH,
            /*height:*/ HEIGHT,
            /*fill:*/ vgc::sRGBA::TRANSPARENT,
            /*stroke:*/ vgc::sRGBA!(Black),
        );

        // Ensure we have accurate information
        if self.ring_damage == 0.0 || !local.is_alive() {
            return;
        }

        // When downed player will have 20hp when revived (TODO: Gold Backpack revive)
        let local_health = if local.is_downed() {
            20.0
        } else {
            local.health as f32
        };

        // Number of ticks we can take before we are dead (+1)
        let n_ticks = f32::ceil(local_health / self.ring_damage);

        // Pop a heal item before this timer runs out
        let timer = (n_ticks - fract) * TICK_TIME;

        let (phoenix, medkit, syringe);
        let (heal_item, heal_time) = if timer > 10.0 {
            (s!(phoenix = "PHOENIX"), 10.0)
        } else if timer > 8.0 {
            (s!(medkit = "MEDKIT"), 8.0)
        } else {
            (s!(syringe = "SYRINGE"), 5.0)
        };

        api.r_text(
			/*font:*/ 0,
			/*flags:*/ 3,
			/*x:*/ xcenter - WIDTH * 0.5 + 4.0,
			/*y:*/ ypos + 11.0,
			/*width:*/ WIDTH,
			/*height:*/ HEIGHT,
			/*color:*/ vgc::sRGBA!(White),
			/*shadow:*/ vgc::sRGBA!(Black),
			/*text:*/ &fmtools::format!({heal_item}" in "{timer - heal_time:.1}"s (hp:"{local.health}" / -"{self.ring_damage}")"),
		);
    }
}
