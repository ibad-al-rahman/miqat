use super::adjustments::TimeAdjustment;
use super::high_latitude_rule::HighLatitudeRule;
use super::mazhab::Mazhab;
use super::method::Method;
use super::rounding::Rounding;
use miqat::IshaaParameter;

/// Flat, FFI-friendly view of [`miqat::Parameters`].
///
/// Ishaa is exposed as a plain angle only. Interval-based Ishaa (e.g. Umm al-Qura's
/// fixed 90 minutes) cannot be expressed here and reads back as `ishaa_angle == 0.0`
/// when querying such a method via [`parameters_for_method`].
#[derive(uniffi::Record)]
pub struct CalculationParameters {
    /// Solar angle below the horizon for Fajr, in degrees.
    pub fajr_angle: f64,
    /// Solar angle below the horizon for Ishaa, in degrees.
    pub ishaa_angle: f64,
    /// Juristic school affecting the Asr calculation.
    pub mazhab: Mazhab,
    /// Rule for approximating Fajr and Ishaa at high latitudes.
    pub high_latitude_rule: HighLatitudeRule,
    /// User-supplied per-prayer offsets, in minutes.
    pub adjustments: TimeAdjustment,
    /// Preset per-prayer offsets baked into a method (populated when querying a method).
    pub method_adjustments: TimeAdjustment,
    /// Output rounding behaviour.
    pub rounding: Rounding,
}

impl From<&CalculationParameters> for miqat::Parameters {
    fn from(params: &CalculationParameters) -> Self {
        miqat::Parameters {
            fajr_angle: params.fajr_angle,
            ishaa_parameter: IshaaParameter::Angle(params.ishaa_angle),
            mazhab: params.mazhab,
            high_latitude_rule: params.high_latitude_rule,
            adjustments: params.adjustments,
            method_adjustments: params.method_adjustments,
            rounding: params.rounding,
            ..Default::default()
        }
    }
}

impl From<miqat::Parameters> for CalculationParameters {
    fn from(params: miqat::Parameters) -> Self {
        let ishaa_angle = match params.ishaa_parameter {
            IshaaParameter::Angle(angle) => angle,
            IshaaParameter::Interval(_) => 0.0,
        };
        CalculationParameters {
            fajr_angle: params.fajr_angle,
            ishaa_angle,
            mazhab: params.mazhab,
            high_latitude_rule: params.high_latitude_rule,
            adjustments: params.adjustments,
            method_adjustments: params.method_adjustments,
            rounding: params.rounding,
        }
    }
}

/// Resolves a preset [`Method`] into its concrete calculation parameters.
///
/// For example, `Method::MuslimWorldLeague` yields `fajr_angle: 18.0`,
/// `ishaa_angle: 17.0`, and `method_adjustments.dhuhr: 1`.
#[uniffi::export]
pub fn parameters_for_method(method: Method) -> CalculationParameters {
    method.parameters().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn muslim_world_league_info() {
        let params = parameters_for_method(Method::MuslimWorldLeague);

        assert_eq!(params.fajr_angle, 18.0);
        assert_eq!(params.ishaa_angle, 17.0);
        assert_eq!(params.method_adjustments.dhuhr, 1);
    }

    #[test]
    fn umm_al_qura_interval_ishaa_reads_back_as_zero() {
        let params = parameters_for_method(Method::UmmAlQura);

        assert_eq!(params.fajr_angle, 18.5);
        assert_eq!(params.ishaa_angle, 0.0);
    }

    #[test]
    fn round_trips_through_core_parameters() {
        let ffi = CalculationParameters {
            fajr_angle: 17.5,
            ishaa_angle: 16.0,
            mazhab: Mazhab::Hanafi,
            high_latitude_rule: HighLatitudeRule::SeventhOfTheNight,
            adjustments: TimeAdjustment {
                fajr: -2,
                ..Default::default()
            },
            method_adjustments: TimeAdjustment {
                dhuhr: 1,
                ..Default::default()
            },
            rounding: Rounding::Ceil,
        };

        let core: miqat::Parameters = (&ffi).into();
        let back: CalculationParameters = core.into();

        assert_eq!(back.fajr_angle, 17.5);
        assert_eq!(back.ishaa_angle, 16.0);
        assert_eq!(back.mazhab, Mazhab::Hanafi);
        assert_eq!(back.high_latitude_rule, HighLatitudeRule::SeventhOfTheNight);
        assert_eq!(back.adjustments.fajr, -2);
        assert_eq!(back.method_adjustments.dhuhr, 1);
        assert_eq!(back.rounding, Rounding::Ceil);
    }
}
