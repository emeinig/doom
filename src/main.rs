use doom::cli::parse_date;
use doom::doomsday::compute_day_of_week;
use std::process;

fn main() {
    let date = parse_date().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1)
    });

    let day_of_week = match compute_day_of_week(&date) {
        0 => "Sunday",
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Sunday",
        _ => "Something has gone horribly wrong",
    };

    println!(
        "{}-{}-{} is a {}",
        date.year, date.month, date.day, day_of_week
    );
}
