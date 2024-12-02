use std::env;
use utils;
use day01;
use day02;
use dayXX;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} {} <dayXX>", args[0], args[1]);
        std::process::exit(1);
    }
    let day = args[2].as_str();

    let input_filepath = utils::get_input_file(&day);
    println!("Reading input from {}", input_filepath);
    let input: Vec<String> = utils::lines_from_file(&input_filepath);

    match day {
        "day01" => run_day(day01::part1, day01::part2, "day01", &input),
        "day02" => run_day(day02::part1, day02::part2, "day02", &input),
        "dayXX" => run_day(dayXX::part1, dayXX::part2, "dayXX", &input),
        _ => {
            eprintln!("Invalid day specified");
            std::process::exit(1);
        }
    }
}
fn run_day(part1: fn(&Vec<String>) -> i64, part2: fn(&Vec<String>) -> i64, day: &str, input: &Vec<String>) {
    println!("Running {}", day);
    let result1 = part1(input);
    println!("Part 1: {}", result1);
    let result2 = part2(input);
    println!("Part 2: {}", result2);
}