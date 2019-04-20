mod year2020;
mod year2021;

fn main() {
    let should_run_all = false;
    year2020::run_year(should_run_all);
    year2021::run_year(should_run_all);
}
