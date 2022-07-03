use clap::Parser;
use doom::date::Date;
use doom::doomsday::compute_day_of_week;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    date: Option<String>,
}

fn parse_date() -> Option<Date> {
    // let string_date = Cli::try_parse().unwrap().date;
    let cli = Cli::try_parse().ok()?;

    if let Some(date_string) = cli.date {
        dbg!(&date_string);
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

fn main() {
    let date = parse_date().unwrap();
    let day_of_week = compute_day_of_week(&date);

    println!("{}-{}-{} is a {}", date.year, date.month, date.day, day_of_week);
}
