mod aoc2020;
mod puzzle;

fn main() {
    println!("Start advent of code challenge");

    run_solutions();
    run_current();
}

fn run_current() {
    aoc2020::day09::challenge();
}

#[allow(dead_code)]
fn run_solutions() {
    aoc2020::day01::challenge();
    aoc2020::day02::challenge();
    aoc2020::day03::challenge();
    aoc2020::day04::challenge();
    aoc2020::day05::challenge();
    aoc2020::day06::challenge();
    aoc2020::day07::challenge();
    aoc2020::day08::challenge();
}
