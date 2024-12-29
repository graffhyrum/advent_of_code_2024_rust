use crate::problems::Problem;
use clap::{arg, command, value_parser, ArgAction, Command};
use std::path::PathBuf;

mod days;
mod problems;

fn main() {
    let matches = command!()
        .arg(
            arg!([day])
                .value_parser(value_parser!(usize))
        )
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("test")
                .about("controls testing features")
                .arg(arg!(-l --list "list test functions").action(ArgAction::SetTrue)),
        )
        .get_matches();

    if let Some(day) = matches.get_one::<usize>("day") {
        if let Some(problem) = day_to_problem(*day) {
            let input_path = format!("src/inputs/day_{}.txt", day);
            if !std::path::Path::new(&input_path).exists() {
                eprintln!("Input file for day {} does not exist", day);
                std::process::exit(1);
            }
            let input = std::fs::read_to_string(&input_path)
                .unwrap_or_else(|_| panic!("Could not read input file for day {}", day));
            println!("Part one: {}", problem.part_one(&input));
            println!("Part two: {}", problem.part_two(&input));
        } else {
            eprintln!("Day {} not implemented", day);
        }
    } else {
        eprintln!("No day provided");
    }

    match matches
        .get_one::<u8>("debug")
        .expect("Counts are defaulted")
    {
        0 => println!("Debugging is off"),
        1 => println!("Debugging is kind of on"),
        2 => println!("Debugging is on"),
        _ => unreachable!("Debugging is either 0, 1, or 2"),
    }

    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.get_flag("list") {
            println!("Listing test functions");
        } else {
            println!("Running tests");
        }
    }

    fn day_to_problem(day: usize) -> Option<Box<dyn Problem>> {
        match day {
            10 => Some(Box::new(days::DayTen)),
            // ...
            _ => None,
        }
    }
}
