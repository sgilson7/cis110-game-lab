//! Calendar dates as whole days, and moments as whole minutes, counted from
//! 1 January 1970. The core reads no clock: the page passes the current date
//! and hour in as text, the way a seed comes in, and these functions turn it
//! into integers. The conversion is Howard Hinnant's `days_from_civil`, which
//! uses integer arithmetic only.

/// Minutes in one day.
pub const DAY_MINUTES: i64 = 24 * 60;

/// A calendar date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year: i64,
    pub month: i64,
    pub day: i64,
}

/// Days from 1970-01-01 to the given date.
pub fn days_from_civil(d: Date) -> i64 {
    let y = if d.month <= 2 { d.year - 1 } else { d.year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (d.month + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d.day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// The date that is `z` days after 1970-01-01.
pub fn civil_from_days(z: i64) -> Date {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + if month <= 2 { 1 } else { 0 };
    Date { year, month, day }
}

/// Day of the week, Monday = 0 through Sunday = 6. 1970-01-01 was a Thursday.
pub fn weekday(days: i64) -> usize {
    (days + 3).rem_euclid(7) as usize
}

fn number(text: &str, what: &str, whole: &str) -> Result<i64, String> {
    if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
        return Err(format!(
            "\"{whole}\" is not a date in the form YYYY-MM-DD ({what})."
        ));
    }
    text.parse::<i64>()
        .map_err(|_| format!("\"{whole}\" is not a date in the form YYYY-MM-DD ({what})."))
}

fn days_in_month(year: i64, month: i64) -> i64 {
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    match month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// Parses `YYYY-MM-DD` into a day number, refusing dates that do not exist.
pub fn parse_day(text: &str) -> Result<i64, String> {
    let parts: Vec<&str> = text.split('-').collect();
    if parts.len() != 3 || parts[0].len() != 4 || parts[1].len() != 2 || parts[2].len() != 2 {
        return Err(format!("\"{text}\" is not a date in the form YYYY-MM-DD."));
    }
    let year = number(parts[0], "year", text)?;
    let month = number(parts[1], "month", text)?;
    let day = number(parts[2], "day", text)?;
    if !(1..=12).contains(&month) || day < 1 || day > days_in_month(year, month) {
        return Err(format!("\"{text}\" is not a date on the calendar."));
    }
    Ok(days_from_civil(Date { year, month, day }))
}

/// Parses a moment, `YYYY-MM-DD HH:MM`, with `T` allowed in place of the space
/// and optional seconds, into minutes since 1970-01-01. A date alone means
/// the start of that day.
pub fn parse_moment(text: &str) -> Result<i64, String> {
    let text = text.trim();
    let (date, clock) = match text.find([' ', 'T']) {
        Some(at) => (&text[..at], &text[at + 1..]),
        None => (text, "00:00"),
    };
    let day = parse_day(date)?;
    let parts: Vec<&str> = clock.split(':').collect();
    if parts.len() < 2 || parts.len() > 3 || parts.iter().any(|p| p.len() != 2) {
        return Err(format!("\"{text}\" has no hour and minute in the form HH:MM."));
    }
    let hour = number(parts[0], "hour", text)?;
    let minute = number(parts[1], "minute", text)?;
    if hour > 23 || minute > 59 {
        return Err(format!("\"{text}\" has an hour or minute out of range."));
    }
    Ok(day * DAY_MINUTES + hour * 60 + minute)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_epoch_is_day_zero_and_a_thursday() {
        assert_eq!(parse_day("1970-01-01"), Ok(0));
        assert_eq!(weekday(0), 3);
    }

    #[test]
    fn known_dates_have_their_day_numbers() {
        // Worked by hand: 2000-03-01 is 11017 days after the epoch
        // (30 years of 365 days, 7 leap days to 1999, plus Jan and Feb 2000).
        assert_eq!(parse_day("2000-03-01"), Ok(11_017));
        assert_eq!(
            civil_from_days(11_017),
            Date {
                year: 2000,
                month: 3,
                day: 1
            }
        );
    }

    #[test]
    fn the_syllabus_mondays_are_mondays() {
        // The syllabus calls 19 October, 2 November and 23 November 2026 Mondays.
        for d in ["2026-10-19", "2026-11-02", "2026-11-23"] {
            assert_eq!(weekday(parse_day(d).unwrap()), 0, "{d}");
        }
        // and 10 December 2026 a Thursday.
        assert_eq!(weekday(parse_day("2026-12-10").unwrap()), 3);
    }

    #[test]
    fn a_day_round_trips() {
        for d in ["2026-10-14", "2026-12-31", "2028-02-29"] {
            let n = parse_day(d).unwrap();
            let c = civil_from_days(n);
            assert_eq!(format!("{:04}-{:02}-{:02}", c.year, c.month, c.day), d);
        }
    }

    #[test]
    fn impossible_dates_are_refused() {
        assert!(parse_day("2026-02-29").is_err());
        assert!(parse_day("2026-13-01").is_err());
        assert!(parse_day("2026-1-01").is_err());
        assert!(parse_day("tomorrow").is_err());
    }

    #[test]
    fn moments_read_the_page_clock_and_the_address_override() {
        let day = parse_day("2026-10-26").unwrap();
        assert_eq!(
            parse_moment("2026-10-26 16:59:30"),
            Ok(day * DAY_MINUTES + 16 * 60 + 59)
        );
        assert_eq!(parse_moment("2026-10-26T17:00"), Ok(day * DAY_MINUTES + 17 * 60));
        assert_eq!(parse_moment("2026-10-26"), Ok(day * DAY_MINUTES));
        assert!(parse_moment("2026-10-26 25:00").is_err());
        assert!(parse_moment("2026-10-26 5pm").is_err());
    }
}
