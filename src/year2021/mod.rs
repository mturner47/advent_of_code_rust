mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

pub(crate) fn run_year(should_run_all: bool) {
    println!("Year 2021:");
    if should_run_all {
        day1::run_day_1(false);
        day1::run_day_1(true);
        day2::run_day_2(false);
        day2::run_day_2(true);
        day3::run_day_3(false);
        day3::run_day_3(true);
        day4::run_day_4(false);
        day4::run_day_4(true);
        day5::run_day_5(false);
        day5::run_day_5(true);
        day6::run_day_6(false);
        day6::run_day_6(true);
        day7::run_day_7(false);
        day7::run_day_7(true);
    }
}
