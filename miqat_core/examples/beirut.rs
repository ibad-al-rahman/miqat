use miqat::prelude::*;

fn main() {
    println!("Prayer times for Beirut in UTC");
    println!("------------------------------");
    println!();

    let date = Utc::now().date_naive();
    let hijri = HijriDate::from_gregorian(date);
    println!("Hijri date: {hijri}");
    println!();
    let makka = Coordinates::new(33.888630, 35.495480);
    let params = Method::Egyptian.parameters();
    let prayer_times = PrayerTimes::computed(date, makka, params);

    println!(
        "{:?}: {}",
        Prayer::Fajr,
        prayer_times.time(Prayer::Fajr).format("%-l:%M %p")
    );
    println!(
        "{:?}: {}",
        Prayer::Sunrise,
        prayer_times.time(Prayer::Sunrise).format("%-l:%M %p")
    );
    println!(
        "{:?}: {}",
        Prayer::Dhuhr,
        prayer_times.time(Prayer::Dhuhr).format("%-l:%M %p")
    );
    println!(
        "{:?}: {}",
        Prayer::Asr,
        prayer_times.time(Prayer::Asr).format("%-l:%M %p")
    );
    println!(
        "{:?}: {}",
        Prayer::Maghrib,
        prayer_times.time(Prayer::Maghrib).format("%-l:%M %p")
    );
    println!(
        "{:?}: {}",
        Prayer::Ishaa,
        prayer_times.time(Prayer::Ishaa).format("%-l:%M %p")
    );
}
