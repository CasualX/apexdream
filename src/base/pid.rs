use obfstr::obfstr as s;
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct PidConfig {
    /// Proportional coefficient.
    pub kp: f32,
    /// Integral coefficient.
    pub ki: f32,
    /// Derivative coefficient.
    pub kd: f32,
    /// Damping factor for the integral coefficient.
    pub damp: f32,
}
impl Default for PidConfig {
    fn default() -> PidConfig {
        PidConfig {
            kp: 2.0,
            ki: 10.0,
            kd: 0.0,
            damp: 0.9,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct PidController {
    p: f32,
    i: f32,
}
impl PidController {
    #[inline]
    pub fn step(&mut self, err: f32, dt: f32, config: &PidConfig) -> f32 {
        let d = (err - self.p) / dt;
        self.p = err;
        self.i += err * dt;
        // Damp the integral factor if it disagrees with the proportional factor
        if self.p * self.i <= 0.0 {
            self.i *= config.damp;
        }
        config.kp * self.p + config.ki * self.i + config.kd * d
    }
    pub fn reset(&mut self) {
        self.p = 0.0;
        self.i = 0.0;
    }
}
impl fmt::Debug for PidController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct(s!("PidController"))
            .field(s!("p"), &self.p)
            .field(s!("i"), &self.i)
            .finish()
    }
}
