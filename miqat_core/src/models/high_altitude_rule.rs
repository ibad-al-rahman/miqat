use crate::astronomy::unit::Coordinates;

/// Rule for approximating Fajr and Ishaa at high latitudes
#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum HighLatitudeRule {
    /// Fajr won't be earlier than the midpoint of the night and ishaa
    /// won't be later than the midpoint of the night. This is the default
    /// value to prevent fajr and ishaa crossing boundaries.
    #[default]
    MiddleOfTheNight,

    /// Fajr will never be earlier than the beginning of the last seventh of
    /// the night and Ishaa will never be later than the end of the first seventh of the night.
    ///
    /// This is recommended to use for locations above 48° latitude to prevent prayer
    /// times that would be difficult to perform.
    SeventhOfTheNight,

    /// The night is divided into portions of roughly 1/3. The exact value is derived
    /// by dividing the fajr/ishaa angles by 60.
    ///
    /// This can be used to prevent difficult fajr and ishaa times at certain locations.
    TwilightAngle,
}

impl HighLatitudeRule {
    pub fn recommended(coordinates: Coordinates) -> HighLatitudeRule {
        if coordinates.latitude > 48.0 {
            HighLatitudeRule::SeventhOfTheNight
        } else {
            HighLatitudeRule::MiddleOfTheNight
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recommended_rule_seventh_of_night() {
        let location = Coordinates {
            latitude: 48.983226,
            longitude: -3.216649,
            elevation: 0.0,
        };

        assert_eq!(
            HighLatitudeRule::recommended(location),
            HighLatitudeRule::SeventhOfTheNight
        );
    }

    #[test]
    fn recommended_rule_middle_of_night() {
        let location = Coordinates {
            latitude: 45.983226,
            longitude: -3.216649,
            elevation: 0.0,
        };

        assert_eq!(
            HighLatitudeRule::recommended(location),
            HighLatitudeRule::MiddleOfTheNight
        );
    }
}
