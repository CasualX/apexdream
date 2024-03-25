#[derive(Clone, Default, Debug)]
pub struct EstimateDerivative {
    pub history: Vec<(f64, [f32; 3])>,
}
impl EstimateDerivative {
    pub fn update(&mut self, time: f64, value: [f32; 3], dt: f64) -> [f32; 3] {
        // Remove samples older than time - dt
        let mintime = time - dt;
        self.history.retain(move |&(time, _)| time > mintime);

        // Add a new sample
        self.history.push((time, value));

        // Must have at least 2 samples
        if self.history.len() >= 2 {
            // Get first and last sample
            if let Some(&(oldtime, oldval)) = self.history.first() {
                // Simple linear model, returns derivative `dt * 0.5` seconds ago
                let inv_dt = 1.0 / (time - oldtime) as f32;
                return [
                    (value[0] - oldval[0]) * inv_dt,
                    (value[1] - oldval[1]) * inv_dt,
                    (value[2] - oldval[2]) * inv_dt,
                ];
            }
        }

        // Default to zero derivative
        [0.0; 3]
    }
}
