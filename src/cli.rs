use crate::date::Date;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(value_parser = nonempty_string_or_none)]
    date: Option<String>,
}

fn nonempty_string_or_none(s: &str) -> Result<String, &'static str> {
    if s.trim().is_empty() {
        Err("Empty Date Field")
    } else {
        Ok(s.to_string())
    }
}

pub fn parse_date() -> Result<Date, &'static str> {
    let cli = Cli::parse();

    let date_string = cli.date.ok_or("No Date Given")?;

    date_string_to_date_struct(date_string)
}

fn date_string_to_date_struct(date_string: String) -> Result<Date, &'static str> {
    let date_vec = date_string.trim().split("-").collect::<Vec<&str>>();
    let year = date_vec[2].parse::<isize>();
    let month = date_vec[0].parse::<usize>();
    let day = date_vec[1].parse::<isize>();

    if year.is_ok_and(|&year| year >= 1752) && month.is_ok_and(|&month| month > 0 && month <= 12) && day.is_ok_and(|&day| day > 0 && day <= 31) {
        Ok(Date {
            year: year.unwrap(),
            month: month.unwrap(),
            day: day.unwrap(),
        })
    } else {
        Err("Incorrectly formatted date")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_not_allow_weird_numbers() {
        let test_date = String::from("13-32-1750");
        let expected = Err("Incorrectly formatted date");

        let result = date_string_to_date_struct(test_date);
        assert_eq!(result, expected);
    }

    #[test]
    fn fails_for_letters() {
        let test_date = String::from("09-22-201a");
        let expected = Err("Incorrectly formatted date");

        let result = date_string_to_date_struct(test_date);
        assert_eq!(result, expected);
    }

    #[test]
    fn works_for_date_with_errant_space() {
        let test_date = String::from("09-22-2017 ");
        let expected = Ok(Date {
            year: 2017,
            month: 9,
            day: 22,
        });

        let result = date_string_to_date_struct(test_date);
        assert_eq!(result, expected);
    }

    #[test]
    fn fails_properly_on_empty_string() {
        let test_date = " ";
        let expected = Err("Empty Date Field");

        let result = nonempty_string_or_none(test_date);
        assert_eq!(result, expected);
    }
}
