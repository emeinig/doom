use crate::date::*;
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

#[cfg(test)]
mod tests {
    use crate::cli::*;

    #[test]
    fn fails_properly_on_empty_string() {
        let test_date = " ";
        let expected = Err("Empty Date Field");

        let result = nonempty_string_or_none(test_date);
        assert_eq!(result, expected);
    }
}
