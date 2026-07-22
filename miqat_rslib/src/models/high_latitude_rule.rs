pub type HighLatitudeRule = miqat::HighLatitudeRule;

#[uniffi::remote(Enum)]
pub enum HighLatitudeRule {
    MiddleOfTheNight,
    SeventhOfTheNight,
    TwilightAngle,
}
