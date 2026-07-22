//! An Islamic prayer time implementation based on the [Adhan](https://github.com/batoulapps/Adhan) library by Batoul Apps.
//! While it has a lot of commnalities with the interface has
//! been changed slightly to make it more ergonomic and intuitive.
//!
//! ##### Example
//!
//! ```
//! use miqat::prelude::*;
//!
//! let new_york_city = Coordinates::new(40.7128, -74.0059);
//! let date          = NaiveDate::from_ymd_opt(2019, 1, 25).expect("Invalid date provided");
//! let params        = Configuration::with(Method::NorthAmerica, Mazhab::Hanafi);
//! let prayers       = PrayerSchedule::new()
//!                       .on(date)
//!                       .for_location(new_york_city)
//!                       .with_configuration(params)
//!                       .calculate();
//! ```

mod astronomy;
pub mod hijri;
mod models;
mod prayer_times;
pub mod precomputed;

pub use crate::astronomy::unit::Coordinates;
pub use crate::astronomy::unit::Stride;
pub use crate::hijri::HijriDate;
pub use crate::hijri::IslamicEvent;
pub use crate::models::adjustments::TimeAdjustment;
pub use crate::models::high_altitude_rule::HighLatitudeRule;
pub use crate::models::ishaa_parameter::IshaaParameter;
pub use crate::models::mazhab::Mazhab;
pub use crate::models::method::Method;
pub use crate::models::parameters::Parameters;
pub use crate::models::rounding::Rounding;
pub use crate::models::prayer::Prayer;
pub use crate::prayer_times::PrayerTimes;
pub use crate::precomputed::provider::{Provider, ProviderCity};
pub use chrono::DateTime;
pub use chrono::Datelike;
pub use chrono::Duration;
pub use chrono::Local;
pub use chrono::NaiveDate;
pub use chrono::TimeZone;
pub use chrono::Timelike;
pub use chrono::Utc;

/// A convenience module appropriate for glob imports (`use miqat::prelude::*;`).
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::astronomy::qiblah::Qiblah;
    #[doc(no_inline)]
    pub use crate::astronomy::unit::{Coordinates, Stride};
    #[doc(no_inline)]
    pub use crate::hijri::HijriDate;
    #[doc(no_inline)]
    pub use crate::hijri::IslamicEvent;
    #[doc(no_inline)]
    pub use crate::hijri::events::{IslamicEventOccurrence, events_for_gregorian_year};
    #[doc(no_inline)]
    pub use crate::models::adjustments::TimeAdjustment;
    #[doc(no_inline)]
    pub use crate::models::high_altitude_rule::HighLatitudeRule;
    #[doc(no_inline)]
    pub use crate::models::ishaa_parameter::IshaaParameter;
    #[doc(no_inline)]
    pub use crate::models::mazhab::Mazhab;
    #[doc(no_inline)]
    pub use crate::models::method::Method;
    #[doc(no_inline)]
    pub use crate::models::parameters::Parameters;
    #[doc(no_inline)]
    pub use crate::models::rounding::Rounding;
    #[doc(no_inline)]
    pub use crate::models::prayer::Prayer;
    #[doc(no_inline)]
    pub use crate::prayer_times::PrayerTimes;
    #[doc(no_inline)]
    pub use crate::precomputed::provider::{Provider, ProviderCity};
    #[doc(no_inline)]
    pub use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, TimeZone, Timelike, Utc};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::high_altitude_rule::HighLatitudeRule;
    use chrono::prelude::*;

    #[test]
    fn calculate_prayer_times() {
        let local_date = NaiveDate::from_ymd_opt(2015, 7, 12).expect("Invalid date provided");
        let params = Method::NorthAmerica.parameters().mazhab(Mazhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let schedule = PrayerTimes::computed(local_date, coordinates, params);

        assert_eq!(
            schedule.time(Prayer::Fajr).format("%-l:%M %p").to_string(),
            "8:42 AM"
        );
        assert_eq!(
            schedule
                .time(Prayer::Sunrise)
                .format("%-l:%M %p")
                .to_string(),
            "10:08 AM"
        );
        assert_eq!(
            schedule.time(Prayer::Dhuhr).format("%-l:%M %p").to_string(),
            "5:21 PM"
        );
        assert_eq!(
            schedule.time(Prayer::Asr).format("%-l:%M %p").to_string(),
            "10:22 PM"
        );
        assert_eq!(
            schedule
                .time(Prayer::Maghrib)
                .format("%-l:%M %p")
                .to_string(),
            "12:32 AM"
        );
        assert_eq!(
            schedule.time(Prayer::Ishaa).format("%-l:%M %p").to_string(),
            "1:57 AM"
        );
    }

    #[test]
    fn calculate_times() {
        let date = NaiveDate::from_ymd_opt(2015, 7, 12).expect("Invalid date provided");
        let params = Method::NorthAmerica.parameters().mazhab(Mazhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let prayer_times = PrayerTimes::computed(date, coordinates, params);
        assert_eq!(
            prayer_times
                .time(Prayer::Fajr)
                .format("%-l:%M %p")
                .to_string(),
            "8:42 AM"
        );
        assert_eq!(
            prayer_times
                .time(Prayer::Sunrise)
                .format("%-l:%M %p")
                .to_string(),
            "10:08 AM"
        );
        assert_eq!(
            prayer_times
                .time(Prayer::Dhuhr)
                .format("%-l:%M %p")
                .to_string(),
            "5:21 PM"
        );
        assert_eq!(
            prayer_times
                .time(Prayer::Asr)
                .format("%-l:%M %p")
                .to_string(),
            "10:22 PM"
        );
        assert_eq!(
            prayer_times
                .time(Prayer::Maghrib)
                .format("%-l:%M %p")
                .to_string(),
            "12:32 AM"
        );
        assert_eq!(
            prayer_times
                .time(Prayer::Ishaa)
                .format("%-l:%M %p")
                .to_string(),
            "1:57 AM"
        );
    }

    #[test]
    fn calculate_times_for_singapore() {
        let mut params = Method::Singapore.parameters().mazhab(Mazhab::Shafi);

        params.high_latitude_rule = HighLatitudeRule::MiddleOfTheNight;

        let prayer_times = PrayerTimes::computed(
            NaiveDate::from_ymd_opt(2021, 1, 13).expect("Invalid date provided"),
            Coordinates::new(1.370844612058886, 103.80145644060552),
            params,
        );

        let hour = 3600;
        let sgt_offset = FixedOffset::east_opt(8 * hour).expect("Invalid offset provided");
        let sgt_fajr = prayer_times.time(Prayer::Fajr).with_timezone(&sgt_offset);
        let sgt_sunrise = prayer_times
            .time(Prayer::Sunrise)
            .with_timezone(&sgt_offset);
        let sgt_dhuhr = prayer_times.time(Prayer::Dhuhr).with_timezone(&sgt_offset);
        let sgt_asr = prayer_times.time(Prayer::Asr).with_timezone(&sgt_offset);
        let sgt_maghrib = prayer_times
            .time(Prayer::Maghrib)
            .with_timezone(&sgt_offset);
        let sgt_isha = prayer_times.time(Prayer::Ishaa).with_timezone(&sgt_offset);

        assert_eq!(sgt_fajr.format("%-l:%M %p").to_string(), "5:50 AM");
        assert_eq!(sgt_sunrise.format("%-l:%M %p").to_string(), "7:13 AM");
        assert_eq!(sgt_dhuhr.format("%-l:%M %p").to_string(), "1:15 PM");
        assert_eq!(sgt_asr.format("%-l:%M %p").to_string(), "4:39 PM");
        assert_eq!(sgt_maghrib.format("%-l:%M %p").to_string(), "7:16 PM");
        assert_eq!(sgt_isha.format("%-l:%M %p").to_string(), "8:30 PM");
    }
}
