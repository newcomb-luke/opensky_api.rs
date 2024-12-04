#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub lat_min: f32,
    pub lat_max: f32,
    pub long_min: f32,
    pub long_max: f32,
}

impl BoundingBox {
    pub fn new(lat_min: f32, lat_max: f32, long_min: f32, long_max: f32) -> Self {
        Self {
            lat_min,
            lat_max,
            long_min,
            long_max,
        }
    }
}
