use std::env;
use day01;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} {} <dayXX>", args[0], args[1]);
        std::process::exit(1);
    }

    match args[2].as_str() {
        "day01" => run_day(day01::part1, day01::part2, "Day 01"),
        _ => {
            eprintln!("Invalid day specified");
            std::process::exit(1);
        }
    }
}
fn run_day(part1: fn() -> i64, part2: fn() -> i64, day: &str) {
    println!("Running {}", day);
    let result1 = part1();
    println!("Part 1: {}", result1);
    let result2 = part2();
    println!("Part 2: {}", result2);
}