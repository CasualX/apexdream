use super::*;

#[derive(Default)]
pub struct DeathField {
    pub is_active: bool,
    pub origin: [f32; 3],
    pub radius_start: f32,
    pub radius_end: f32,
    pub time_start: f32,
    pub time_end: f32,
}
impl DeathField {
    pub fn radius(&self, curtime: f32) -> f32 {
        if self.time_end == self.time_start {
            return 0.0;
        }
        let fraction = ((curtime - self.time_start) / (self.time_end - self.time_start))
            .max(0.0)
            .min(1.0);
        self.radius_start + fraction * (self.radius_end - self.radius_start)
    }
    /// Distance to the death field.
    ///
    /// Positive if outside the zone, negative if inside the zone.
    pub fn distance(&self, curtime: f32, target: [f32; 3]) -> f32 {
        if self.time_end == self.time_start {
            return 0.0;
        }
        sdk::dist(target, self.origin) - self.radius(curtime)
    }
    /// Returns if the target is inside the zone.
    pub fn is_inside(&self, curtime: f32, target: [f32; 3]) -> bool {
        self.distance(curtime, target) <= 0.0
    }
    /// Returns if the death field is zero.
    pub fn is_zero(&self) -> bool {
        return self.is_active == false
            && self.origin == [0.0; 3]
            && self.radius_start == 0.0
            && self.radius_end == 0.0
            && self.time_start == 0.0
            && self.time_end == 0.0;
    }
}

#[derive(Default)]
pub struct WorldEntity {
    pub entity_ptr: sdk::Ptr,
    pub entity_size: u32,
    pub index: u32,
    update_rate: u32,
    update_time: f64,
    pub death_field: DeathField,
}
impl WorldEntity {
    pub fn new(entity_ptr: sdk::Ptr, index: u32, cc: &sdk::ClientClass) -> Box<dyn Entity> {
        let entity_size = cc.ClassSize;
        Box::new(WorldEntity {
            entity_ptr,
            entity_size,
            index,
            ..WorldEntity::default()
        }) as Box<dyn Entity>
    }
}
impl Entity for WorldEntity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_ref(&self) -> EntityRef<'_> {
        EntityRef::World(self)
    }
    fn is_serialized(&self) -> bool {
        true
    }
    fn get_info(&self) -> EntityInfo {
        EntityInfo {
            entity_ptr: self.entity_ptr,
            index: self.index as usize,
            handle: sdk::EHandle::from(self.index),
            rate: self.update_rate,
        }
    }
    fn update(&mut self, api: &mut Api, ctx: &UpdateContext) {
        // Each DeathField array is 0x40 in size
        let base = ctx.data.world_death_field;
        let mut indices = [
            // m_deathFieldIsActive
            base,
            // m_deathFieldOrigin
            base + 1 * 0x40 + 0,
            base + 1 * 0x40 + 4,
            base + 1 * 0x40 + 8,
            // m_deathFieldRadiusStart
            base + 1 * 0x40 + 12 * 0x40,
            // m_deathFieldRadiusEnd
            base + 1 * 0x40 + 12 * 0x40 + 4 * 0x40,
            // m_deathFieldTimeStart
            base + 1 * 0x40 + 12 * 0x40 + 4 * 0x40 + 4 * 0x40,
            // m_deathFieldTimeEnd
            base + 1 * 0x40 + 12 * 0x40 + 4 * 0x40 + 4 * 0x40 + 4 * 0x40,
        ];
        if let Ok(fields) = api.vm_gatherd(self.entity_ptr, self.entity_size, &mut indices) {
            self.death_field.is_active = fields[0] & 0xff != 0;
            let last_origin = self.death_field.origin;
            self.death_field.origin[0] = f32::from_bits(fields[1]);
            self.death_field.origin[1] = f32::from_bits(fields[2]);
            self.death_field.origin[2] = f32::from_bits(fields[3]);
            if self.death_field.origin != last_origin {
                self.update_time = ctx.time;
            }
            self.death_field.radius_start = f32::from_bits(fields[4]);
            self.death_field.radius_end = f32::from_bits(fields[5]);
            self.death_field.time_start = f32::from_bits(fields[6]);
            self.death_field.time_end = f32::from_bits(fields[7]);
        }

        self.update_rate = if ctx.time >= self.update_time + 0.25 {
            64
        } else {
            2
        };
    }
}
