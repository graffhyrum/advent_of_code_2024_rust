use crate::problems::Problem;
use clap::{arg, command, value_parser};
use std::fs;
use std::path::Path;

mod days;
mod problems;
mod util;

fn main() {
    let matches = command!()
        .arg(arg!([day]).value_parser(value_parser!(usize)))
        .get_matches();

    if let Some(&day) = matches.get_one::<usize>("day") {
        if let Some(problem) = day_to_problem(day) {
            let input_path = format!("src/inputs/day_{}.txt", day);
            if !Path::new(&input_path).exists() {
                eprintln!("Input file for day {} does not exist", day);
                std::process::exit(1);
            }
            let input = fs::read_to_string(&input_path)
                .unwrap_or_else(|_| panic!("Could not read input file for day {}", day));
            println!("Part one: {}", problem.part_one(&input));
            println!("Part two: {}", problem.part_two(&input));
        } else {
            eprintln!("Day {} not implemented", day);
        }
    } else {
        eprintln!("No day provided");
    }
}

fn day_to_problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(days::DayOne)),
        2 => Some(Box::new(days::DayTwo)),
        3 => Some(Box::new(days::DayThree)),
        4 => Some(Box::new(days::DayFour)),
        5 => Some(Box::new(days::DayFive)),
        6 => Some(Box::new(days::DaySix)),
        7 => Some(Box::new(days::DaySeven)),
        8 => Some(Box::new(days::DayEight)),
        9 => Some(Box::new(days::DayNine)),
        10 => Some(Box::new(days::DayTen)),
        11 => Some(Box::new(days::DayEleven)),
        12 => Some(Box::new(days::DayTwelve)),
        13 => Some(Box::new(days::DayThirteen)),
        14 => Some(Box::new(days::DayFourteen)),
        15 => Some(Box::new(days::DayFifteen)),
        16 => Some(Box::new(days::DaySixteen)),
        17 => Some(Box::new(days::DaySeventeen)),
        18 => Some(Box::new(days::DayEighteen)),
        19 => Some(Box::new(days::DayNineteen)),
        20 => Some(Box::new(days::DayTwenty)),
        21 => Some(Box::new(days::DayTwentyOne)),
        22 => Some(Box::new(days::DayTwentyTwo)),
        23 => Some(Box::new(days::DayTwentyThree)),
        24 => Some(Box::new(days::DayTwentyFour)),
        25 => Some(Box::new(days::DayTwentyFive)),
        _ => None,
    }
}
