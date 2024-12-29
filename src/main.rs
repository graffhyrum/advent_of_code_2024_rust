use crate::problems::Problem;
use clap::{arg, command, value_parser};
use std::fs;
use std::path::Path;

mod days;
mod problems;

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
        10 => Some(Box::new(days::DayTen)),
        _ => None,
    }
}
