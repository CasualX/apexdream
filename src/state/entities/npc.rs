use super::*;

#[derive(Default)]
pub struct BaseNPCEntity {
    pub entity_ptr: sdk::Ptr,
    pub entity_size: u32,
    pub index: u32,

    pub origin: [f32; 3],
    pub angles: [f32; 3],

    pub model_name: ModelName,
    pub studio: StudioModel,
    pub bones: super::BoneArray,

    pub skin: i32,
    pub skin_mod: i32,
    pub body: i32,
    pub camo_index: i32,

    pub last_visible_time: f32,
    pub is_visible: bool,
    pub visible_time: f64,
}
impl BaseNPCEntity {
    pub fn new(entity_ptr: sdk::Ptr, index: u32, cc: &sdk::ClientClass) -> Box<dyn Entity> {
        let entity_size = cc.ClassSize;
        Box::new(BaseNPCEntity {
            entity_ptr,
            entity_size,
            index,
            ..BaseNPCEntity::default()
        }) as Box<dyn Entity>
    }
    pub fn get_bone_pos(&self, bone: usize) -> [f32; 3] {
        sdk::add(self.origin, self.bones.get_pos(bone))
    }
    pub fn height(&self) -> f32 {
        60.0
    }
}
impl Entity for BaseNPCEntity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_ref(&self) -> EntityRef<'_> {
        EntityRef::BaseNPC(self)
    }
    fn is_serialized(&self) -> bool {
        false
    }
    fn get_info(&self) -> EntityInfo {
        EntityInfo {
            entity_ptr: self.entity_ptr,
            index: self.index as usize,
            handle: sdk::EHandle::from(self.index),
            rate: 1,
        }
    }
    fn update(&mut self, api: &mut Api, ctx: &UpdateContext) {
        #[derive(sdk::Pod)]
        #[repr(C)]
        struct Indices {
            origin: [u32; 6],
            model_name: [u32; 2],
            bone_array: [u32; 2],
            studio: [u32; 2],
            skin: [u32; 4],
            last_visible_time: u32,
        }

        let data = ctx.data;
        let mut indices = Indices {
            origin: [
                data.entity_origin + 0,
                data.entity_origin + 4,
                data.entity_origin + 8,
                data.entity_origin + 24,
                data.entity_origin + 28,
                data.entity_origin + 32,
            ],
            model_name: [data.entity_model_name + 0, data.entity_model_name + 4],
            bone_array: [data.animating_bone_array + 0, data.animating_bone_array + 4],
            studio: [data.animating_studiohdr + 0, data.animating_studiohdr + 4],
            skin: [
                data.animating_skin + 0,
                data.animating_skin + 4,
                data.animating_skin + 8,
                data.animating_skin + 12,
            ],
            last_visible_time: data.bcc_last_visible_time,
        };

        if let Ok(fields) = api.vm_gatherd(self.entity_ptr, self.entity_size, &mut indices) {
            self.origin[0] = f32::from_bits(fields.origin[0]);
            self.origin[1] = f32::from_bits(fields.origin[1]);
            self.origin[2] = f32::from_bits(fields.origin[2]);
            self.angles[0] = f32::from_bits(fields.origin[3]);
            self.angles[1] = f32::from_bits(fields.origin[4]);
            self.angles[2] = f32::from_bits(fields.origin[5]);

            let model_name_ptr = fields.model_name[0] as u64 | (fields.model_name[1] as u64) << 32;
            self.model_name.update(api, model_name_ptr.into());
            let studio_ptr = fields.studio[0] as u64 | (fields.studio[1] as u64) << 32;
            self.studio.update(api, sdk::Ptr::from_raw(studio_ptr));
            let bones_ptr = fields.bone_array[0] as u64 | (fields.bone_array[1] as u64) << 32;
            self.bones
                .update(api, ctx, &self.studio, sdk::Ptr::from_raw(bones_ptr));

            self.skin = fields.skin[0] as i32;
            self.skin_mod = fields.skin[1] as i32;
            self.body = fields.skin[2] as i32;
            self.camo_index = fields.skin[3] as i32;

            self.last_visible_time = f32::from_bits(fields.last_visible_time);
        }
    }
    fn post(&mut self, _api: &mut Api, ctx: &UpdateContext, state: &GameState) {
        // Check if npc is visible
        let is_visible = self.last_visible_time > 0.0
            && (self.last_visible_time - state.client.curtime).abs() < 0.1;
        // Take note when the npc became visible
        if !self.is_visible && is_visible {
            self.visible_time = ctx.time;
        }
        self.is_visible = is_visible;
    }
}
