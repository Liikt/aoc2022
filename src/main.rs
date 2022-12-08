use std::env::args;
use std::process::exit;
use std::fs::read_to_string;

macro_rules! day {
    ($day:ident, $day_str:expr) => {
        use $day;
        println!("{} part 1: {}", $day_str, $day::solve1(
            read_to_string("inputs/".to_owned() + $day_str).unwrap()));
        println!("{} part 2: {}", $day_str, $day::solve2(
            read_to_string("inputs/".to_owned() + $day_str).unwrap()));
    };
}

fn main() {
    if args().len() != 2 {
        println!("Usage: ./{} <all|dayX>", args().next().unwrap());
        exit(1);
    }

    match args().nth(1).unwrap().as_str() {
        "all" => {
            day!(day1, "day1");
            day!(day2, "day2");
            day!(day3, "day3");
            day!(day4, "day4");
            day!(day5, "day5");
            day!(day6, "day6");
            day!(day7, "day7");
            day!(day8, "day8");
        },
        "day1" => {
            day!(day1, "day1");
        }
        "day2" => {
            day!(day2, "day2");
        }
        "day3" => {
            day!(day3, "day3");
        }
        "day4" => {
            day!(day4, "day4");
        }
        "day5" => {
            day!(day5, "day5");
        }
        "day6" => {
            day!(day6, "day6");
        }
        "day7" => {
            day!(day7, "day7");
        }
        "day8" => {
            day!(day8, "day8");
        }

        _ => panic!("Invalid day as argument")
    }
}
