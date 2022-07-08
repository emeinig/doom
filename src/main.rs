use doom::cli::parse_date;
use doom::doomsday::compute_day_of_week;
use doom::iso_week_date::week_number;
use std::process;

fn main() {
    let date = parse_date().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1)
    });

    let day_of_week = compute_day_of_week(&date);

    let iso_week_date = week_number(&date, &day_of_week);

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
        "{}-{}-{} is a {}\nThe ISO week date is {}-W{}-{}",
        date.year,
        date.month,
        date.day,
        human_readable_day_of_week,
        iso_week_date.year,
        iso_week_date.week_of_year,
        iso_week_date.day,
    );
}
