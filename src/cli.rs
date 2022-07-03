use crate::date::Date;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(value_parser)]
    date: Option<String>,
}

pub fn parse_date() -> Option<Date> {
    let cli = Cli::try_parse().ok()?;

    if let Some(date_string) = cli.date {
        let date_vec = date_string.split("-").collect::<Vec<&str>>();

        Some(Date {
            year: date_vec[2].parse::<isize>().unwrap(),
            month: date_vec[0].parse::<usize>().unwrap(),
            day: date_vec[1].parse::<isize>().unwrap(),
        })
    } else {
        None
    }
}
