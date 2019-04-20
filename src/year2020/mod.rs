mod day1;
mod day2;
mod day3;
mod day4;

pub(crate) fn run_year(should_run_all: bool) {
    println!("Year 2020:");
    if should_run_all {
        day1::run_day(false);
        day1::run_day(true);
        day2::run_day(false);
        day2::run_day(true);
        day3::run_day(false);
        day3::run_day(true);
        day4::run_day(false);
        day4::run_day(true);
    }
}
