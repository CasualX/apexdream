use crate::sdk::HighlightSetting;
use crate::state::entity_list::EntityList;
use super::*;

pub struct HighlightSettings {
    offset: u32,
    settings: Box<[HighlightSetting]>
}
impl Default for HighlightSettings {
    fn default() -> HighlightSettings{
        HighlightSettings {
            offset: 0,
            settings: vec![HighlightSetting::default(); 255].into_boxed_slice(),
        }
    }
}
impl HighlightSettings {
    pub fn  update(&mut self, api: &mut Api, ctx: &UpdateContext) {
        let _ = api.vm_read_into(ctx.process.base.field(ctx.data.highlight_settings), &mut *self.settings);
    }
}