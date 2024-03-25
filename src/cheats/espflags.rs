use crate::*;
use std::{fmt, str};

/// ESP feature flags.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Flags(pub u16);
impl Flags {
    pub const NONE: Flags = Flags(0);
    /// Draw bounds.
    pub const BOUNDS: Flags = Flags(1 << 0);
    /// Draw 3D box.
    pub const BOX: Flags = Flags(1 << 1);
    /// Draw text.
    pub const TEXT: Flags = Flags(1 << 2);
    /// Draw name.
    pub const NAME: Flags = Flags(1 << 3);
    /// Draw health bar.
    pub const HEALTH: Flags = Flags(1 << 4);
    /// Draw icon.
    pub const ICON: Flags = Flags(1 << 5);
    /// Draw bones.
    pub const BONES: Flags = Flags(1 << 6);
    /// Draw trail.
    pub const TRAIL: Flags = Flags(1 << 7);
    /// Draw spine.
    pub const SPINE: Flags = Flags(1 << 8);
    /// Draw barrel.
    pub const BARREL: Flags = Flags(1 << 9);
    /// Draw where to aim.
    pub const AIM: Flags = Flags(1 << 10);
    /// Draw half alpha.
    pub const FADED: Flags = Flags(1 << 11);
    /// Draw with fixed alpha.
    pub const ALPHA: Flags = Flags(1 << 12);
    /// Draw square at origin.
    pub const ORIGIN: Flags = Flags(1 << 13);
    /// Draw dot in the sky.
    pub const SKYDOT: Flags = Flags(1 << 14);
}
impl std::ops::BitOr for Flags {
    type Output = Flags;
    #[inline]
    fn bitor(self, rhs: Flags) -> Flags {
        Flags(self.0 | rhs.0)
    }
}
impl std::ops::BitAnd for Flags {
    type Output = bool;
    #[inline]
    fn bitand(self, rhs: Flags) -> bool {
        self.0 & rhs.0 != 0
    }
}
impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let &flags = self;
        fmtools::write! { f,
            if (flags == Flags::NONE) { "none " }
            else {
                if (flags & Flags::BOUNDS) { "bounds " }
                if (flags & Flags::BOX) { "box " }
                if (flags & Flags::TEXT) { "text " }
                if (flags & Flags::NAME) { "name " }
                if (flags & Flags::HEALTH) { "health " }
                if (flags & Flags::ICON) { "icon " }
                if (flags & Flags::BONES) { "bones " }
                if (flags & Flags::TRAIL) { "trail " }
                if (flags & Flags::SPINE) { "spine " }
                if (flags & Flags::BARREL) { "barrel " }
                if (flags & Flags::AIM) { "aim " }
                if (flags & Flags::FADED) { "faded " }
                if (flags & Flags::ALPHA) { "alpha " }
                if (flags & Flags::ORIGIN) { "origin " }
                if (flags & Flags::SKYDOT) { "skydot " }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct UnknownFlagError;

impl fmt::Display for UnknownFlagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmtools::write!(f, "unknown flag")
    }
}

impl std::error::Error for UnknownFlagError {}

impl std::str::FromStr for Flags {
    type Err = UnknownFlagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut flags = 0u16;
        for s in s.split_ascii_whitespace() {
            if s == s!("none") {
                flags |= Flags::NONE.0;
            } else if s == s!("bounds") {
                flags |= Flags::BOUNDS.0
            } else if s == s!("box") {
                flags |= Flags::BOX.0
            } else if s == s!("text") {
                flags |= Flags::TEXT.0
            } else if s == s!("name") {
                flags |= Flags::NAME.0
            } else if s == s!("health") {
                flags |= Flags::HEALTH.0
            } else if s == s!("icon") {
                flags |= Flags::ICON.0
            } else if s == s!("bones") {
                flags |= Flags::BONES.0
            } else if s == s!("trail") {
                flags |= Flags::TRAIL.0
            } else if s == s!("spine") {
                flags |= Flags::SPINE.0
            } else if s == s!("barrel") {
                flags |= Flags::BARREL.0
            } else if s == s!("aim") {
                flags |= Flags::AIM.0
            } else if s == s!("faded") {
                flags |= Flags::FADED.0
            } else if s == s!("alpha") {
                flags |= Flags::ALPHA.0
            } else if s == s!("origin") {
                flags |= Flags::ORIGIN.0
            } else if s == s!("skydot") {
                flags |= Flags::SKYDOT.0
            } else {
                return Err(UnknownFlagError);
            }
        }
        Ok(Flags(flags))
    }
}
