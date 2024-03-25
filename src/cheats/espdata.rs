use crate::base::math;
use crate::state::StudioModel;
use crate::*;

pub use super::espflags::Flags;

#[derive(Default)]
pub struct Config {
    pub debug_bones: bool,
    pub debug_models: bool,
    pub fade_distance: f32,
    pub fade_factor: f32,
    pub icon_image: u32,
    pub icon_grid: f32,
    pub bounds_scale: f32,
    pub bounds_trans: f32,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Icon {
    pub x: u8,
    pub y: u8,
}

#[derive(Clone, Default)]
pub struct Object<'a> {
    /// Drawing flags.
    pub flags: Flags,
    /// Player name associated with this object.
    pub name: Option<&'a str>,
    /// Legend/Class name of the object.
    pub text: Option<&'a str>,
    /// Is the object visible.
    pub visible: bool,
    /// Primary drawing color.
    pub color: vgc::sRGB,
    /// Fade distance and alpha.
    pub fade_dist: f32,
    pub alpha: f32,
    /// Object center in the world.
    pub origin: [f32; 3],
    /// Object view vector.
    pub view: [f32; 3],
    /// Object spine offsets.
    pub spine: [[f32; 3]; 2],
    /// Aim here to hit the predicted target.
    pub aim: Option<[f32; 3]>,
    /// Aim here to skynade.
    pub skynade_pitch: f32,
    pub skynade_yaw: f32,
    /// Cached distance to the camera.
    pub distance: f32,
    /// Bone matrices.
    pub bones: Option<&'a [[f32; 12]]>,
    /// Studio model
    pub studio: Option<&'a StudioModel>,
    /// Trail.
    pub trail: Option<&'a [[f32; 3]]>,
    /// Width of the object in the world.
    pub width: f32,
    /// Height of the object in the world.
    pub height: f32,
    /// Icon assicated with this object.
    pub icon: Option<Icon>,
    /// Health and shields of the object.
    pub health: i32,
    pub max_health: i32,
    pub shields: i32,
    pub max_shields: i32,
    /// Debug model name.
    pub model_name: Option<&'a str>,
    pub skin: i32,
}

impl<'a> Object<'a> {
    pub fn draw(&self, api: &mut Api, ctx: &RunContext, conf: &Config) {
        draw_object(self, conf, api, ctx);
    }
}

fn alpha(dist: f32, max: f32, factor: f32) -> Option<f32> {
    if dist >= max {
        return None;
    }
    let a = f32::atan((1.0 - dist / max) * factor) / f32::atan(factor);
    let b = a.min(1.0).max(0.0);
    return Some(b);
}

#[inline(never)]
fn draw_object(object: &Object, conf: &Config, api: &mut Api, ctx: &RunContext) {
    let alpha = if object.flags & Flags::ALPHA {
        object.alpha
    } else {
        match alpha(object.distance, object.fade_dist, conf.fade_factor) {
            Some(a) => a,
            None => return,
        }
    };
    let alpha = (alpha * 255.0) as u8;
    let color = object.color.alpha(alpha);
    let light = vgc::sRGBA(255, 255, 255, alpha);
    let dark = vgc::sRGBA(0, 0, 0, alpha / 2 + alpha / 4);
    let nearby = math::dist2(ctx.camera_origin(), object.origin) < 2048.0 * 2048.0;

    if object.flags & Flags::TRAIL {
        if let Some(trail) = object.trail {
            for i in 0..trail.len() {
                // let alpha = ((i as i32 - trail.len() as i32) as f32 / self.config.trail_fade + 1.0).max(0.0).min(1.0) * alpha;
                let v1 = *trail.get(i).unwrap_or(&object.origin);
                let v2 = *trail.get(i + 1).unwrap_or(&object.origin);
                if let Some([x1, y1]) = ctx.world_to_screen(v1, true) {
                    if let Some([x2, y2]) = ctx.world_to_screen(v2, true) {
                        api.r_line(color, x1, y1, x2, y2);
                    }
                }
            }
        }
    }

    if object.skynade_pitch != 0.0 {
        // let delta = sdk::sub(target, local.view_origin);
        // let dx = f32::sqrt(delta[0] * delta[0] + delta[1] * delta[1]);
        // let dir = [delta[0] / dx, delta[1] / dx, pitch.tan()];
        // let point = sdk::add(sdk::mul(dir, [1000.0; 3]), local.view_origin);

        if let Some([px, py]) = ctx.angles_to_screen(
            [
                -object.skynade_pitch.to_degrees(),
                object.skynade_yaw.to_degrees(),
                0.0,
            ],
            false,
        ) {
            let px = px.round();
            let py = py.round();
            if let Some([sx, sy]) = ctx.world_to_screen(object.origin, false) {
                api.r_line(
                    /*color:*/ vgc::sRGBA!(White),
                    /*x1:*/ sx.round(),
                    /*y1:*/ sy.round(),
                    /*x2:*/ px,
                    /*y2:*/ py,
                );
            }
            api.r_ellipse(
                /*x:*/ px - 3.0,
                /*y:*/ py - 3.0,
                /*width:*/ 6.0,
                /*height:*/ 6.0,
                /*fill:*/ vgc::sRGBA(0xff, 0, 0x66, 255),
                /*stroke:*/ vgc::sRGBA!(Black),
            );
            api.r_text(
                /*font:*/ 0,
                /*flags:*/ 3,
                px,
                py,
                /*width:*/ 1000.0,
                /*height:*/ 100.0,
                /*color*/ vgc::sRGBA!(White),
                /*shadow:*/ vgc::sRGBA!(Black),
                /*text:*/ &fmtools::format!({object.skynade_pitch.to_degrees():.1}"°"),
            );
        }
    }

    if conf.debug_bones && nearby {
        if let Some(bones) = object.bones {
            if let Some(studio) = object.studio {
                for bbox in &studio.hitboxes {
                    let matrix = match bones.get(bbox.bone as usize) {
                        Some(matrix) => matrix,
                        None => continue,
                    };
                    let pos = math::add(object.origin, [matrix[3], matrix[7], matrix[11]]);
                    if let Some([x, y]) = ctx.world_to_screen(pos, true) {
                        api.r_text(
                            /*font:*/ 0,
                            /*flags:*/ 3,
                            x,
                            y,
                            /*width:*/ 1000.0,
                            /*height:*/ 100.0,
                            color,
                            /*shadow:*/ dark,
                            /*text:*/ &bbox.bone.to_string(),
                        );
                        let radius = bbox.radius();
                        if let Some([x2, y2]) =
                            ctx.world_to_screen([pos[0], pos[1], pos[2] + radius], true)
                        {
                            let r = f32::sqrt((x - x2) * (x - x2) + (y - y2) * (y - y2));
                            api.r_ellipse(
                                x - r,
                                y - r,
                                r + r,
                                r + r,
                                vgc::sRGBA::TRANSPARENT,
                                color,
                            );
                        }
                    }
                }
            } else {
                for (id, matrix) in bones.iter().enumerate() {
                    let pos = math::add(object.origin, [matrix[3], matrix[7], matrix[11]]);
                    if let Some([x, y]) = ctx.world_to_screen(pos, true) {
                        api.r_text(
                            /*font:*/ 0,
                            /*flags:*/ 3,
                            x,
                            y,
                            /*width:*/ 1000.0,
                            /*height:*/ 100.0,
                            color,
                            /*shadow:*/ dark,
                            /*text:*/ &id.to_string(),
                        );
                    }
                }
            }
        }
    }

    if object.flags & Flags::BONES {
        if let Some(studio) = object.studio {
            let draw_line = |api: &mut Api, bbox: &sdk::mstudiobbox_t| {
                let Some(parent) = studio.parent_hitbox(bbox) else {
                    return;
                };
                let Some(bones) = object.bones else { return };
                let start = match bones.get(bbox.bone as usize) {
                    Some(start) => start,
                    _ => return,
                };
                let end = match bones.get(parent.bone as usize) {
                    Some(end) => end,
                    _ => return,
                };
                let start = math::add(object.origin, [start[3], start[7], start[11]]);
                let end = math::add(object.origin, [end[3], end[7], end[11]]);

                if let Some([x1, y1]) = ctx.world_to_screen(start, true) {
                    if let Some([x2, y2]) = ctx.world_to_screen(end, true) {
                        api.r_line(color, x1, y1, x2, y2);
                    }
                }
            };
            if ctx.full_bones {
                for bbox in &studio.hitboxes {
                    draw_line(api, bbox);
                }
            } else {
                for bbox in studio.spine() {
                    draw_line(api, bbox);
                }
            }
        }
    }

    if object.flags & Flags::BOX {
        let hwidth = object.width * 0.5;
        let compute_pts = || {
            Some([
                ctx.world_to_screen(math::add(object.origin, [-hwidth, -hwidth, 0.0]), true)?,
                ctx.world_to_screen(math::add(object.origin, [-hwidth, hwidth, 0.0]), true)?,
                ctx.world_to_screen(math::add(object.origin, [hwidth, hwidth, 0.0]), true)?,
                ctx.world_to_screen(math::add(object.origin, [hwidth, -hwidth, 0.0]), true)?,
                ctx.world_to_screen(
                    math::add(object.origin, [-hwidth, -hwidth, object.height]),
                    true,
                )?,
                ctx.world_to_screen(
                    math::add(object.origin, [-hwidth, hwidth, object.height]),
                    true,
                )?,
                ctx.world_to_screen(
                    math::add(object.origin, [hwidth, hwidth, object.height]),
                    true,
                )?,
                ctx.world_to_screen(
                    math::add(object.origin, [hwidth, -hwidth, object.height]),
                    true,
                )?,
            ])
        };
        if let Some(pts) = compute_pts() {
            static LINES: [[u16; 2]; 12] = [
                [0, 1],
                [1, 2],
                [2, 3],
                [3, 0],
                [4, 5],
                [5, 6],
                [6, 7],
                [7, 4],
                [0, 4],
                [1, 5],
                [2, 6],
                [3, 7],
            ];
            api.r_lines(color, &pts, &LINES);
        }
    }

    if object.flags & Flags::ORIGIN {
        if let Some([x, y]) = ctx.world_to_screen(object.origin, true) {
            api.r_rect(
                /*x:*/ x - 2.0,
                /*y:*/ y - 2.0,
                /*width:*/ 4.0,
                /*height:*/ 4.0,
                /*fill:*/ color,
                /*stroke:*/ dark,
            );
        }
    }

    if object.flags & Flags::SPINE && nearby {
        let p1 = math::add(object.origin, object.spine[0]);
        let p2 = math::add(object.origin, object.spine[1]);

        if let Some([x1, y1]) = ctx.world_to_screen(p1, true) {
            if let Some([x2, y2]) = ctx.world_to_screen(p2, true) {
                api.r_line(color, x1, y1, x2, y2);
            }
        }
    }

    if object.flags & Flags::BARREL {
        let p1 = math::add(object.origin, object.spine[0]);
        let p2 = math::add(p1, math::muls(object.view, 40.0));

        if let Some([x1, y1]) = ctx.world_to_screen(p1, true) {
            if let Some([x2, y2]) = ctx.world_to_screen(p2, true) {
                api.r_line(color, x1, y1, x2, y2);
            }
        }
    }

    if (object.flags & Flags::AIM) && object.visible {
        if let Some(aim) = object.aim {
            if let Some([x, y]) = ctx.angles_to_screen(aim, true) {
                let x = x.round();
                let y = y.round();
                api.r_rect(
                    /*x:*/ x - 4.0,
                    /*y:*/ y - 1.0,
                    /*width:*/ 9.0,
                    /*height:*/ 3.0,
                    /*fill:*/ dark,
                    /*stroke:*/ vgc::sRGBA::TRANSPARENT,
                );
                api.r_rect(
                    /*x:*/ x - 1.0,
                    /*y:*/ y - 4.0,
                    /*width:*/ 3.0,
                    /*height:*/ 9.0,
                    /*fill:*/ dark,
                    /*stroke:*/ vgc::sRGBA::TRANSPARENT,
                );
                api.r_line(color, x - 3.0, y, x + 3.0, y);
                api.r_line(color, x, y - 3.0, x, y + 3.0);
            }
        }
    }

    let Some(bounds) = bounds(object, conf, ctx) else {
        return;
    };

    if object.flags & Flags::TEXT {
        if let Some(text) = if conf.debug_models {
            object.model_name
        } else {
            object.text
        } {
            if let Some([x, y]) = ctx.world_to_screen(object.origin, true) {
                api.r_text(
                    /*font:*/ 0, /*flags:*/ 3, x, y, /*width:*/ 1000.0,
                    /*height:*/ 100.0, color, /*shadow:*/ dark, text,
                );
            }
        }
    }

    if object.flags & Flags::NAME {
        if let Some(name) = object.name {
            if let Some([x, y]) = ctx.world_to_screen(object.origin, true) {
                api.r_text(
                    /*font:*/ 0, /*flags:*/ 3, x, y, /*width:*/ 1000.0,
                    /*height:*/ 100.0, color, /*shadow:*/ dark, /*text:*/ name,
                );
            }
        }
    }

    if object.flags & Flags::HEALTH {
        let width = bounds.right - bounds.left;
        if width >= 12.0 {
            let height = 3.0;
            let x = bounds.left;
            let y = bounds.bottom + 2.0;
            api.r_rect(
                /*x:*/ x - 0.5,
                /*y:*/ y - 0.5,
                /*width:*/ width + 1.0,
                /*height:*/ height + 1.0,
                /*fill:*/ dark,
                /*stroke:*/ vgc::sRGBA::TRANSPARENT,
            );
            if object.max_health > 0 {
                let health = f32::min(1.0, object.health as f32 / object.max_health as f32);
                let width = width * health;
                api.r_rect(
                    x,
                    y,
                    width,
                    height,
                    /*fill:*/ light,
                    /*stroke:*/ vgc::sRGBA::TRANSPARENT,
                );
            }
            if object.max_shields > 0 {
                let shields = f32::min(1.0, object.shields as f32 / object.max_shields as f32);
                let width = width * shields;
                api.r_rect(
                    /*x:*/ x + 0.5,
                    /*y:*/ y + 0.5,
                    /*width:*/ width - 1.0,
                    /*height:*/ height - 1.0,
                    /*fill:*/ color,
                    /*stroke:*/ vgc::sRGBA::TRANSPARENT,
                );
            }
        }
    }

    if object.flags & Flags::ICON {
        if let Some(icon) = object.icon {
            let size = icon_size(object.distance, 500.0, 1500.0, 24.0, 32.0).round();
            let x = bounds.cx;
            let y = bounds.cy;
            api.r_image(
                /*image:*/ conf.icon_image,
                /*sx:*/ icon.x as f32 * conf.icon_grid,
                /*sy:*/ icon.y as f32 * conf.icon_grid,
                /*swidth:*/ conf.icon_grid,
                /*sheight:*/ conf.icon_grid,
                /*dx:*/ x - size * 0.5,
                /*dy:*/ y - size * 0.5,
                /*dwidth:*/ size,
                /*dheight:*/ size,
                /*opacity:*/ 0.8,
            );
        }
    }

    if object.flags & Flags::BOUNDS {
        let width = bounds.right - bounds.left;
        let height = bounds.bottom - bounds.top;
        if object.visible {
            api.r_rect(
                /*x:*/ bounds.left + 1.0,
                /*y:*/ bounds.top + 1.0,
                width,
                height,
                /*fill:*/ vgc::sRGBA::TRANSPARENT,
                /*stroke:*/ dark,
            );
            api.r_rect(
                /*x:*/ bounds.left,
                /*y:*/ bounds.top,
                width,
                height,
                /*fill:*/ vgc::sRGBA::TRANSPARENT,
                /*stroke:*/ color,
            );
            api.r_text(
                /*font:*/ 0, /*flags:*/ 3,                 /*x:*/ bounds.left,
                /*y:*/ bounds.top, /*width:*/ 1000.0,
                /*height:*/ 100.0, color, /*shadow:*/ dark, /*text:*/ "vis",
            );
        } else {
            let size = f32::min(width, height) / 4.0;
            let width = width - 1.0;
            let height = height - 1.0;
            let x = bounds.left + 1.0;
            let y = bounds.top + 1.0;
            let mut points = [
                [x, y],
                [x, y + size],
                [x + size, y],
                [x + width, y],
                [x + width, y + size],
                [x + width - size, y],
                [x, y + height],
                [x, y + height - size],
                [x + size, y + height],
                [x + width, y + height],
                [x + width, y + height - size],
                [x + width - size, y + height],
            ];
            static LINES: [[u16; 2]; 8] = [
                [0, 1],
                [0, 2],
                [3, 4],
                [3, 5],
                [6, 7],
                [6, 8],
                [9, 10],
                [9, 11],
            ];
            api.r_lines(
                /*color:*/ dark, /*points:*/ &points, /*lines:*/ &LINES,
            );
            for p in &mut points {
                p[0] -= 1.0;
                p[1] -= 1.0;
            }
            api.r_lines(color, /*points:*/ &points, /*lines:*/ &LINES);
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ObjectBounds {
    pub cx: f32,
    pub cy: f32,
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[inline(never)]
fn bounds(object: &Object, conf: &Config, ctx: &RunContext) -> Option<ObjectBounds> {
    // For objects which don't specify their width or height assume a point
    if object.width <= 0.0 || object.height <= 0.0 {
        let [cx, cy] = ctx.world_to_screen(object.origin, true)?;
        return Some(ObjectBounds {
            cx,
            cy,
            left: cx,
            top: cy,
            right: cx,
            bottom: cy,
        });
    }

    // This logic is necessary to make the bounds behave nice looking nearly up/down

    let camera_origin = ctx.camera_origin();
    let hwidth = object.width * 0.5;

    let dir = math::sub(camera_origin, object.origin);
    let dir = math::norm([dir[0], dir[1], 0.0]);
    let vbot1 = math::add(object.origin, math::muls(dir, hwidth));
    let vbot2 = math::add(object.origin, math::muls(dir, -hwidth));
    let bot1 = ctx.world_to_screen(vbot1, true)?;
    let bot2 = ctx.world_to_screen(vbot2, true)?;

    let vtop = [
        object.origin[0],
        object.origin[1],
        object.origin[2] + object.height,
    ];
    let dir = math::sub(camera_origin, vtop);
    let dir = math::norm([dir[0], dir[1], 0.0]);
    let vtop1 = math::add(vtop, math::muls(dir, hwidth));
    let vtop2 = math::add(vtop, math::muls(dir, -hwidth));
    let top1 = ctx.world_to_screen(vtop1, true)?;
    let top2 = ctx.world_to_screen(vtop2, true)?;

    let y1 = bot1[1].min(bot2[1]).min(top1[1]).min(top2[1]);
    let y2 = bot1[1].max(bot2[1]).max(top1[1]).max(top2[1]);
    let ph = y2 - y1;
    let pw = ph * (object.width / object.height);

    let [cx, cy] = ctx.world_to_screen(
        [
            object.origin[0],
            object.origin[1],
            object.origin[2] + object.height * 0.5,
        ],
        true,
    )?;
    let width = pw * 0.5 * conf.bounds_scale + conf.bounds_trans;
    let height = ph * 0.5 * conf.bounds_scale + conf.bounds_trans;
    Some(ObjectBounds {
        cx,
        cy,
        left: cx - width,
        right: cx + width,
        top: cy - height,
        bottom: cy + height,
    })
}

// Perspective correct icon sizes
fn icon_size(d: f32, near: f32, far: f32, min: f32, max: f32) -> f32 {
    if d > far {
        return min;
    }
    if d < near {
        return max;
    }
    let a_near = f32::atan(1.0 / near);
    let a_far = f32::atan(1.0 / far);
    let alpha = f32::atan(1.0 / d);
    let ratio = (alpha - a_far) / (a_near - a_far);
    ratio * (max - min) + min
}
