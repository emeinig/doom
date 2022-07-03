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

    let date_vec = date_string.trim().split("-").collect::<Vec<&str>>();
    let year = date_vec[2].parse::<isize>();
    let month = date_vec[0].parse::<usize>();
    let day = date_vec[1].parse::<isize>();

    if year.is_ok() && month.is_ok() && day.is_ok() {
        Ok(Date {
            year: year.unwrap(),
            month: month.unwrap(),
            day: day.unwrap(),
        })
    } else {
        Err("Incorrectly formatted date")
    }
}
