pub type TimeAdjustment = miqat::TimeAdjustment;

#[uniffi::remote(Record)]
pub struct TimeAdjustment {
    pub fajr: i64,
    pub sunrise: i64,
    pub dhuhr: i64,
    pub asr: i64,
    pub maghrib: i64,
    pub ishaa: i64,
}
