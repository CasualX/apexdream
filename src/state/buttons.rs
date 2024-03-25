use super::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct Buttons {
    pub in_attack: sdk::kbutton_t,
    pub in_jump: sdk::kbutton_t,
    pub in_duck: sdk::kbutton_t,
    pub in_reload: sdk::kbutton_t,
    pub in_use: sdk::kbutton_t,
    pub in_zoom: sdk::kbutton_t,
    pub in_forward: sdk::kbutton_t,
    pub in_backward: sdk::kbutton_t,
    pub in_moveleft: sdk::kbutton_t,
    pub in_moveright: sdk::kbutton_t,
}

impl Buttons {
    pub fn update(&mut self, api: &mut Api, ctx: &UpdateContext) {
        self.read(api, ctx.process, ctx.data);
        // clear_input_state at input + 0xF0 (int)
    }
    pub fn read(&mut self, api: &mut Api, process: &GameProcess, data: &GameData) {
        let mut indices = [
            data.in_attack + 0,
            data.in_attack + 4,
            data.in_attack + 8,
            data.in_jump + 0,
            data.in_jump + 4,
            data.in_jump + 8,
            data.in_duck + 0,
            data.in_duck + 4,
            data.in_duck + 8,
            data.in_reload + 0,
            data.in_reload + 4,
            data.in_reload + 8,
            data.in_use + 0,
            data.in_use + 4,
            data.in_use + 8,
            data.in_zoom + 0,
            data.in_zoom + 4,
            data.in_zoom + 8,
            data.in_forward + 0,
            data.in_forward + 4,
            data.in_forward + 8,
            data.in_backward + 0,
            data.in_backward + 4,
            data.in_backward + 8,
            data.in_moveleft + 0,
            data.in_moveleft + 4,
            data.in_moveleft + 8,
            data.in_moveright + 0,
            data.in_moveright + 4,
            data.in_moveright + 8,
        ];

        if let Ok(fields) = api.vm_gatherd(process.base, process.size_of_image, &mut indices) {
            let fields = dataview::DataView::from(fields).read::<[sdk::kbutton_t; 10]>(0);
            self.in_attack = fields[0];
            self.in_jump = fields[1];
            self.in_duck = fields[2];
            self.in_reload = fields[3];
            self.in_use = fields[4];
            self.in_zoom = fields[5];
            self.in_forward = fields[6];
            self.in_backward = fields[7];
            self.in_moveleft = fields[8];
            self.in_moveright = fields[9];
        }
    }
}

#[allow(dead_code)]
impl GameState {
    pub fn in_attack_state(&self) -> bool {
        self.buttons.in_attack.state & 1 != 0
    }
    pub fn in_attack_down(&self) -> bool {
        let down = self.buttons.in_attack.down;
        down[0] | down[1] != 0
    }
    pub fn in_reload(&self) -> bool {
        self.buttons.in_reload.state & 1 != 0
    }
    pub fn in_jump(&self) -> bool {
        self.buttons.in_jump.state & 1 != 0
    }
    pub fn in_use(&self) -> bool {
        self.buttons.in_use.down[0] | self.buttons.in_use.down[1] != 0
    }
    pub fn in_zoom(&self) -> bool {
        self.buttons.in_zoom.down[0] | self.buttons.in_zoom.down[1] != 0
    }
    pub fn in_forward(&self) -> bool {
        self.buttons.in_forward.state & 1 != 0
    }
    pub fn in_backward(&self) -> bool {
        self.buttons.in_backward.state & 1 != 0
    }
    pub fn in_moveleft(&self) -> bool {
        self.buttons.in_moveleft.state & 1 != 0
    }
    pub fn in_moveright(&self) -> bool {
        self.buttons.in_moveright.state & 1 != 0
    }
}
