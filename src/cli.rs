use crate::date::Date;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(value_parser = nonempty_string_or_none)]
    date: Option<String>,
    #[clap(short, long, value_parser = endian_format)]
    format: Option<DateFormat>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum DateFormat {
    // Big endian or year first (e.g. ISO 8601)
    BigEndian,
    // Little endian or day first (e.g. DD-MM-YYYY)
    LittleEndian,
    // Middle endian or month first (e.g. MM-DD-YYYY)
    MiddleEndian,
}

fn nonempty_string_or_none(s: &str) -> Result<String, &'static str> {
    if s.trim().is_empty() {
        Err("Empty Date Field")
    } else {
        Ok(s.to_string())
    }
}

fn endian_format(s: &str) -> Result<DateFormat, &'static str> {
    match s {
        "iso8601" => Ok(DateFormat::BigEndian),
        "YMD" => Ok(DateFormat::BigEndian),
        "MYD" => Ok(DateFormat::MiddleEndian),
        "DMY" => Ok(DateFormat::LittleEndian),
        "" => Ok(DateFormat::MiddleEndian),
        _ => Err("Date format is not recognized or not given"),
    }
}

pub fn parse_date() -> Result<Date, &'static str> {
    let cli = Cli::parse();

    let date_string = cli.date.ok_or("No Date Given")?;

    match cli.format {
        Some(DateFormat::BigEndian) => big_endian_to_date_struct(date_string),
        Some(DateFormat::MiddleEndian) => middle_endian_to_date_struct(date_string),
        Some(DateFormat::LittleEndian) => little_endian_to_date_struct(date_string),
        _ => middle_endian_to_date_struct(date_string),
    }
}

fn build_date_struct(
    string_year: &str,
    string_month: &str,
    string_day: &str,
) -> Result<Date, &'static str> {
    let year = string_year.parse::<isize>();
    let month = string_month.parse::<usize>();
    let day = string_day.parse::<isize>();

    if year.is_ok_and(|&year| year >= 1752)
        && month.is_ok_and(|&month| month > 0 && month <= 12)
        && day.is_ok_and(|&day| day > 0 && day <= 31)
    {
        Ok(Date {
            year: year.unwrap(),
            month: month.unwrap(),
            day: day.unwrap(),
        })
    } else {
        Err("Incorrectly formatted date")
    }
}

fn big_endian_to_date_struct(date_string: String) -> Result<Date, &'static str> {
    let date_vec = date_string.trim().split("-").collect::<Vec<&str>>();
    let year = date_vec[0];
    let month = date_vec[1];
    let day = date_vec[2];

    build_date_struct(year, month, day)
}

fn middle_endian_to_date_struct(date_string: String) -> Result<Date, &'static str> {
    let date_vec = date_string.trim().split("-").collect::<Vec<&str>>();
    let year = date_vec[2];
    let month = date_vec[0];
    let day = date_vec[1];

    build_date_struct(year, month, day)
}

fn little_endian_to_date_struct(date_string: String) -> Result<Date, &'static str> {
    let date_vec = date_string.trim().split("-").collect::<Vec<&str>>();
    let year = date_vec[2];
    let month = date_vec[1];
    let day = date_vec[0];

    build_date_struct(year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_the_same_struct() {
        let big_endian = String::from("1970-2-1");
        let middle_endian = String::from("2-1-1970");
        let little_endian = String::from("1-2-1970");

        let big_date = big_endian_to_date_struct(big_endian);
        let middle_date = middle_endian_to_date_struct(middle_endian);
        let little_date = little_endian_to_date_struct(little_endian);

        let expected = Ok(Date {
            year: 1970,
            month: 2,
            day: 1,
        });

        assert_eq!(big_date, expected);
        assert_eq!(middle_date, expected);
        assert_eq!(little_date, expected);
    }

    #[test]
    fn does_not_allow_weird_numbers() {
        let test_date = String::from("13-32-1750");
        let expected = Err("Incorrectly formatted date");

        let result = middle_endian_to_date_struct(test_date);
        assert_eq!(result, expected);
    }

    #[test]
    fn fails_for_letters() {
        let test_date = String::from("09-22-201a");
        let expected = Err("Incorrectly formatted date");

        let result = middle_endian_to_date_struct(test_date);
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

        let result = middle_endian_to_date_struct(test_date);
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
