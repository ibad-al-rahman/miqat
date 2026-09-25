pub type Coordinates = miqat::Coordinates;

#[uniffi::remote(Record)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
    /// Elevation above sea level, in metres. Corrects sunrise and sunset for
    /// the horizon dip; pass 0.0 for sea level.
    pub elevation: f64,
}
