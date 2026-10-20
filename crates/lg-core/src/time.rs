use alloc::{format, string::String};

/// Exibido no horário de Brasília (UTC-3, sem horário de verão desde 2019).
/// O valor gravado no save continua em UTC.
pub fn timestamp(ms: u64) -> String {
    const BRT_OFFSET_SECS: i64 = -3 * 3600;
    let seconds = (ms / 1000).min(253_402_300_799) as i64 + BRT_OFFSET_SECS;
    let days = seconds.div_euclid(86400).min(2_932_896);
    let secs_of_day = seconds.rem_euclid(86400);
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let mut year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    let names = [
        "jan", "fev", "mar", "abr", "mai", "jun", "jul", "ago", "set", "out", "nov", "dez",
    ];
    format!(
        "{day} {} {year}  {:02}:{:02}",
        names[month as usize - 1],
        secs_of_day / 3600,
        secs_of_day / 60 % 60
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn save_dates_handle_epoch_and_leap_day() {
        assert_eq!(timestamp(0), "31 dez 1969  21:00");
        assert_eq!(timestamp(10_800_000), "1 jan 1970  00:00");
        assert_eq!(timestamp(951827696000), "29 fev 2000  09:34");
        assert_eq!(timestamp(1790596800000), "28 set 2026  09:00");
        assert_eq!(timestamp(1_792_534_826_000), "20 out 2026  19:20");
    }
}
