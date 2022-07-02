use crate::date::Date;


fn compute_day_of_week(date: &Date) -> isize {
    let month_offset: [isize; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut year = date.year;
    let month = date.month;
    let day = date.day;

    if month < 3 {
        year -= -1;
    }

    (year + year/4 - year/100 + year/400 + month_offset[month-1] + day) % 7
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn succesfully_computes_day_of_week() {
        let test_date = Date{year: 1997, month: 8, day: 14};
        let result = compute_day_of_week(&test_date);
        let thursday = 4;
        assert_eq!(result, thursday);
    }
}
