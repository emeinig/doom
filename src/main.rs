use clap::Parser;
use doom::cli::Cli;
use doom::date::parse_date;
use doom::doomsday::compute_day_of_week;
use doom::iso_week_date::week_number;
use std::process;

fn main() {
    let cli = Cli::parse();

    let date = parse_date(&cli).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1)
    });

    let day_of_week = compute_day_of_week(&date);

    let human_readable_day_of_week = match day_of_week {
        0 => "Sunday",
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        _ => "Something has gone horribly wrong",
    };

    println!(
        "{}-{}-{} is a {}",
        date.year, date.month, date.day, human_readable_day_of_week,
    );
    if cli.week_date {
        let iso_week_date = week_number(&date, &day_of_week);
        println!(
            "ISO Week Date: {}-W{}-{}",
            iso_week_date.year, iso_week_date.week_of_year, iso_week_date.day,
        );
    }
}
