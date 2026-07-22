use super::HijriDate;
use chrono::Datelike;
use chrono::NaiveDate;

/// A well-known Islamic holiday or observance tied to a fixed Hijri date.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IslamicEvent {
    /// 1 Muharram — Islamic New Year
    IslamicNewYear,
    /// 10 Muharram — Day of Ashura
    Ashura,
    /// 12 Rabi' I — Prophet's Birthday (Mawlid al-Nabi)
    MawlidAlNabi,
    /// 23 Rabi' II (583 AH)
    BattleOfHattin,
    /// 15 Jumada I (8 AH)
    BattleOfMutah,
    /// 10 Rajab (9 AH)
    BattleOfTabuk,
    /// 27 Rajab — Night Journey and Ascension (Isra and Mi'raj)
    IsraAndMiraj,
    /// 15 Sha'ban — Middle of Sha'ban (Nisf Sha'ban / Laylat al-Bara'at)
    NisfShaban,
    /// 1 Ramadan — First day of Ramadan
    FirstOfRamadan,
    /// 17 Ramadan (2 AH)
    BattleOfBadr,
    /// 20 Ramadan (8 AH)
    ConquestOfMecca,
    /// 27 Ramadan — Night of Power (Laylat al-Qadr)
    LaylatAlQadr,
    /// 1 Shawwal — Eid al-Fitr
    EidAlFitr,
    /// 4 Shawwal (3 AH)
    BattleOfUhud,
    /// 9 Dhul Hijja — Day of Arafah (Waqfat Arafat)
    DayOfArafah,
    /// 10 Dhul Hijja — Eid al-Adha
    EidAlAdha,
}

impl IslamicEvent {
    pub fn for_date(month: u8, day: u8) -> Vec<IslamicEvent> {
        EVENTS
            .iter()
            .filter(|(m, d, _)| *m == month && *d == day)
            .map(|(_, _, e)| *e)
            .collect()
    }
}

/// An occurrence of an Islamic event with its associated Hijri and Gregorian dates.
#[derive(Debug, Clone)]
pub struct IslamicEventOccurrence {
    pub event: IslamicEvent,
    pub hijri_date: HijriDate,
    pub gregorian_date: chrono::DateTime<chrono::Utc>,
}

/// Returns all recurring Islamic event occurrences that fall within the given Gregorian year,
/// sorted chronologically by Gregorian date.
pub fn events_for_gregorian_year(gregorian_year: i32) -> Vec<IslamicEventOccurrence> {
    let jan1 = NaiveDate::from_ymd_opt(gregorian_year, 1, 1).unwrap();
    let dec31 = NaiveDate::from_ymd_opt(gregorian_year, 12, 31).unwrap();

    let hijri_start = HijriDate::from_gregorian(jan1).year;
    let hijri_end = HijriDate::from_gregorian(dec31).year;

    let mut occurrences = Vec::new();

    for hijri_year in hijri_start..=hijri_end {
        for &(month, day, event) in EVENTS {
            let hijri_date = HijriDate {
                year: hijri_year,
                month,
                day,
            };
            if let Some(gregorian_date) = hijri_date.to_gregorian()
                && gregorian_date.year() == gregorian_year
            {
                occurrences.push(IslamicEventOccurrence {
                    event,
                    hijri_date,
                    gregorian_date,
                });
            }
        }
    }

    occurrences.sort_by_key(|o| o.gregorian_date);
    occurrences
}

/// (month, day, event)
const EVENTS: &[(u8, u8, IslamicEvent)] = &[
    (1, 1, IslamicEvent::IslamicNewYear),
    (1, 10, IslamicEvent::Ashura),
    (3, 12, IslamicEvent::MawlidAlNabi),
    (7, 27, IslamicEvent::IsraAndMiraj),
    (8, 14, IslamicEvent::NisfShaban),
    (9, 1, IslamicEvent::FirstOfRamadan),
    (9, 26, IslamicEvent::LaylatAlQadr), /* 26 instead of 27 to accout for gregorian calendar users */
    (10, 1, IslamicEvent::EidAlFitr),
    (12, 9, IslamicEvent::DayOfArafah),
    (12, 10, IslamicEvent::EidAlAdha),
];

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn returns_ten_events_for_a_typical_year() {
        let occurrences = events_for_gregorian_year(2025);
        assert_that!(occurrences).has_length(10);
    }

    #[test]
    fn all_gregorian_dates_are_within_the_requested_year() {
        for year in [2023, 2024, 2025, 2026] {
            let occurrences = events_for_gregorian_year(year);
            for o in &occurrences {
                assert_that!(o.gregorian_date.year()).is_equal_to(year);
            }
        }
    }

    #[test]
    fn occurrences_are_sorted_chronologically() {
        let occurrences = events_for_gregorian_year(2025);
        let dates: Vec<_> = occurrences.iter().map(|o| o.gregorian_date).collect();
        let mut sorted = dates.clone();
        sorted.sort();
        assert_that!(dates).is_equal_to(sorted);
    }

    #[test]
    fn hijri_dates_round_trip_to_their_gregorian_dates() {
        let occurrences = events_for_gregorian_year(2025);
        for o in &occurrences {
            let converted = o
                .hijri_date
                .to_gregorian()
                .expect("round-trip should succeed");
            assert_that!(converted).is_equal_to(o.gregorian_date);
        }
    }

    #[test]
    fn each_occurrence_hijri_month_and_day_match_the_event() {
        let occurrences = events_for_gregorian_year(2025);
        for o in &occurrences {
            let (expected_month, expected_day) = match o.event {
                IslamicEvent::IslamicNewYear => (1, 1),
                IslamicEvent::Ashura => (1, 10),
                IslamicEvent::MawlidAlNabi => (3, 12),
                IslamicEvent::IsraAndMiraj => (7, 27),
                IslamicEvent::NisfShaban => (8, 14),
                IslamicEvent::FirstOfRamadan => (9, 1),
                IslamicEvent::LaylatAlQadr => (9, 26),
                IslamicEvent::EidAlFitr => (10, 1),
                IslamicEvent::DayOfArafah => (12, 9),
                IslamicEvent::EidAlAdha => (12, 10),
                _ => continue,
            };
            assert_that!(o.hijri_date.month).is_equal_to(expected_month);
            assert_that!(o.hijri_date.day).is_equal_to(expected_day);
        }
    }

    #[test]
    fn eid_al_fitr_2025_falls_in_march() {
        let occurrences = events_for_gregorian_year(2025);
        let eid = occurrences
            .iter()
            .find(|o| o.event == IslamicEvent::EidAlFitr)
            .expect("Eid al-Fitr must be present");

        assert_that!(eid.gregorian_date.month()).is_equal_to(3);
        assert_that!(eid.gregorian_date.day()).is_equal_to(30);
        assert_that!(eid.hijri_date.month).is_equal_to(10);
        assert_that!(eid.hijri_date.day).is_equal_to(1);
    }

    #[test]
    fn eid_al_adha_2025_falls_in_june() {
        let occurrences = events_for_gregorian_year(2025);
        let eid = occurrences
            .iter()
            .find(|o| o.event == IslamicEvent::EidAlAdha)
            .expect("Eid al-Adha must be present");

        assert_that!(eid.gregorian_date.month()).is_equal_to(6);
        assert_that!(eid.gregorian_date.day()).is_equal_to(6);
        assert_that!(eid.hijri_date.month).is_equal_to(12);
        assert_that!(eid.hijri_date.day).is_equal_to(10);
    }
}
