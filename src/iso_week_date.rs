use crate::date::Date;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WeekDate {
    pub year: isize,
    pub week_of_year: isize,
    pub day: isize,
}

pub fn week_number(date: &Date, day_of_week: &isize) -> WeekDate {
    let mut year = date.year;
    let ordinal_date = ordinal_date(*date);
    let weekday_number = if day_of_week == &0 { 7 } else { *day_of_week };

    let week_number = (10 + ordinal_date - weekday_number) / 7;

    let week_of_year = if week_number < 1 {
        year -= 1;
        weeks(year)
    } else if week_number > weeks(year) {
        year += 1;
        1
    } else {
        week_number
    };

    WeekDate {
        year: year,
        week_of_year: week_of_year,
        day: weekday_number,
    }
}

// AKA the day of the year. E.g. Nov 5 = 305th day of the year
fn ordinal_date(date: Date) -> isize {
    let leap_year_offset = if is_leap_year(date) && date.month > 2 {
        1
    } else {
        0
    };

    let month_offset: [isize; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

    leap_year_offset + month_offset[date.month - 1] + date.day
}

fn is_leap_year(date: Date) -> bool {
    (date.year % 4 == 0 && date.year % 100 != 0) || date.year % 400 == 0
}

// finds if year is "short" (has 52 standard ISO weeks) or "long" (has a leap
// week in it). This is needed because we need to check that the date isn't
// in week 1 of the following year.
fn weeks(year: isize) -> isize {
    if is_short(year) {
        52
    } else {
        53
    }
}

fn is_short(year: isize) -> bool {
    let current_year: bool = cassidy_func(year) % 7 == 4;
    let prev_year: bool = cassidy_func(year - 1) % 7 == 3;

    !current_year && !prev_year
}

// Derived in 2001 by Simon Cassidy (United States)
fn cassidy_func(year: isize) -> isize {
    year + (year / 4) - (year / 100) + (year / 400)
}

#[cfg(test)]
mod tests {
    use crate::date::Date;
    use crate::iso_week_date::*;

    #[test]
    fn find_short_years() {
        let short_year = 2003;
        let long_year = 2004;

        assert!(is_short(short_year));
        assert!(!is_short(long_year));
    }

    #[test]
    fn determines_week_date() {
        let date = Date {
            year: 2016,
            month: 11,
            day: 5,
        };
        let saturday = 6;
        let expected = WeekDate {
            year: 2016,
            week_of_year: 44,
            day: saturday,
        };
        let result = week_number(&date, &saturday);
        assert_eq!(result, expected);
    }

    #[test]
    fn determines_week_date_for_next_year() {
        let date = Date {
            year: 1980,
            month: 12,
            day: 29,
        };
        let monday = 1;
        let expected = WeekDate {
            year: 1981,
            week_of_year: 1,
            day: monday,
        };
        let result = week_number(&date, &monday);
        assert_eq!(result, expected);
    }

    #[test]
    fn determines_week_date_for_prev_year() {
        let date = Date {
            year: 1977,
            month: 1,
            day: 2,
        };
        let sunday = 0;
        let expected = WeekDate {
            year: 1976,
            week_of_year: 53,
            day: 7,
        };
        let result = week_number(&date, &sunday);
        assert_eq!(result, expected);
    }

    // let sunday = 0;
    // let result = week_number(&funky_date, &sunday);
    // assert_eq!(result, 53);

    #[test]
    fn determines_ordinal_date() {
        let date = Date {
            year: 2016,
            month: 11,
            day: 5,
        };
        let result = ordinal_date(date);
        assert_eq!(result, 310);
    }

    #[test]
    fn determines_leap_year_correctly() {
        let leap_year = Date {
            year: 2020,
            month: 2,
            day: 22,
        };

        let leap_century = Date {
            year: 2000,
            month: 5,
            day: 29,
        };

        let non_leap_century = Date {
            year: 1900,
            month: 9,
            day: 1,
        };

        let result1 = is_leap_year(leap_year);
        assert!(result1);

        let result2 = is_leap_year(leap_century);
        assert!(result2);

        let result3 = is_leap_year(non_leap_century);
        assert!(!result3);
    }
}
