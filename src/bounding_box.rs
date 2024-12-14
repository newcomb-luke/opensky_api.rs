#[derive(Debug, Clone, Copy)]
/// Represents a certain area defined by a bounding box of WGS84 coordinates.
pub struct BoundingBox {
    /// lower bound for the latitude in decimal degrees
    pub lat_min: f32,
    /// upper bound for the latitude in decimal degrees
    pub lat_max: f32,
    /// lower bound for the longitude in decimal degrees
    pub long_min: f32,
    /// upper bound for the longitude in decimal degrees
    pub long_max: f32,
}

impl BoundingBox {
    /// Creates a new BoundingBox with the given coordinates.
    pub fn new(lat_min: f32, lat_max: f32, long_min: f32, long_max: f32) -> Self {
        Self {
            lat_min,
            lat_max,
            long_min,
            long_max,
        }
    }
}
