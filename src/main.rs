use doom::cli::parse_date;
use doom::doomsday::compute_day_of_week;
use std::process;

fn main() {
    let date = parse_date().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1)
    });
    let day_of_week = compute_day_of_week(&date);

    println!(
        "{}-{}-{} is a {}",
        date.year, date.month, date.day, day_of_week
    );
}
