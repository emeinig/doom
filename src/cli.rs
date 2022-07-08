use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Default format is MM-DD-YYYY. Slashes and periods are not accepted
    #[clap(value_parser = nonempty_string_or_none)]
    pub date: Option<String>,
    /// Accepted formats are "iso8601", "YMD" (year-month-day), "MYD"
    /// (month-year-day), & "DMY" (day-month-year)
    #[clap(short, long, value_parser = endian_format)]
    pub format: Option<DateFormat>,
    /// Prints the ISO Week Date in addition to the day of week
    #[clap(short, long, action)]
    pub week_date: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DateFormat {
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
