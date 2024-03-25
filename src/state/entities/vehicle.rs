use super::*;

#[derive(Default)]
pub struct VehicleEntity {
    pub entity_ptr: sdk::Ptr,
    pub entity_size: u32,
    pub index: u32,

    update_rate: u32,
    update_time: f64,

    pub origin: [f32; 3],
    pub angles: [f32; 3],

    pub vehicle_driver: sdk::EHandle,
    pub vehicle_velocity: [f32; 3],
}
impl VehicleEntity {
    pub fn new(entity_ptr: sdk::Ptr, index: u32, cc: &sdk::ClientClass) -> Box<dyn Entity> {
        let entity_size = cc.ClassSize;
        Box::new(VehicleEntity {
            entity_ptr,
            entity_size,
            index,
            ..VehicleEntity::default()
        }) as Box<dyn Entity>
    }
}
impl Entity for VehicleEntity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_ref(&self) -> EntityRef<'_> {
        EntityRef::Vehicle(self)
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
        #[derive(sdk::Pod)]
        #[repr(C)]
        struct Indices {
            origin: [u32; 6],
            driver: u32,
            vehicle_velocity: [u32; 3],
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
            driver: data.vehicle_driver,
            vehicle_velocity: [
                data.vehicle_velocity + 0,
                data.vehicle_velocity + 4,
                data.vehicle_velocity + 8,
            ],
        };

        if let Ok(fields) = api.vm_gatherd(self.entity_ptr, self.entity_size, &mut indices) {
            let last_origin = self.origin;
            self.origin[0] = f32::from_bits(fields.origin[0]);
            self.origin[1] = f32::from_bits(fields.origin[1]);
            self.origin[2] = f32::from_bits(fields.origin[2]);
            if self.origin != last_origin {
                self.update_time = ctx.time;
            }
            self.angles[0] = f32::from_bits(fields.origin[3]);
            self.angles[1] = f32::from_bits(fields.origin[4]);
            self.angles[2] = f32::from_bits(fields.origin[5]);
            self.vehicle_driver = sdk::EHandle::from(fields.driver);
            self.vehicle_velocity[0] = f32::from_bits(fields.vehicle_velocity[0]);
            self.vehicle_velocity[1] = f32::from_bits(fields.vehicle_velocity[1]);
            self.vehicle_velocity[2] = f32::from_bits(fields.vehicle_velocity[2]);
        }

        self.update_rate = if ctx.time >= self.update_time + 1.0 {
            128
        } else {
            2
        };
    }
}
