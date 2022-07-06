use crate::date::Date;

fn ordinal_date(date: &Date) -> isize {
    let leap_year_offset = if is_leap_year(&date) && date.month > 2 {
        1
    } else {
        0
    };

    let month_offset: [isize; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

    leap_year_offset + month_offset[date.month - 1] + date.day
}

fn is_leap_year(date: &Date) -> bool {
    if (date.year % 4 == 0 && date.year % 100 != 0) || date.year % 400 == 0 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::date::Date;
    use crate::iso_week_date::*;

    #[test]
    fn determines_ordinal_date() {
        let date = Date {
            year: 2016,
            month: 11,
            day: 5,
        };
        let result = ordinal_date(&date);
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

        let result1 = is_leap_year(&leap_year);
        assert!(result1);

        let result2 = is_leap_year(&leap_century);
        assert!(result2);

        let result3 = is_leap_year(&non_leap_century);
        assert!(!result3);
    }
}
