use advent_of_code_2022::aoc::day_1;

macro_rules! read_file {
    ($input:expr) => {
        std::fs::read_to_string($input).unwrap_or_else(|e| panic!("error: {e} not found"))
    };
}
fn main() {
    // Day 1: Calorie Counting
    let input = read_file!([env!("PWD"), "inputs/day1.txt"].join("/"));

    println!("{:?}", day_1(input.as_str()));
}
