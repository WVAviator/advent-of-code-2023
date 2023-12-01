use crate::challenge::ChallengeFactory;
use clap::{command, Arg};
use std::{fs::File, path::Path};

mod challenge;
mod utils;

fn main() {
    let matches = command!("Rust - Advent of Code 2023")
        .version("1.0")
        .author("WVAviator")
        .about("Solves input files from Advent of Code 2023")
        .arg(
            Arg::new("challenge")
                .required(true)
                .index(1)
                .help("The challenge number from Advent of Code 2023"),
        )
        .arg(
            Arg::new("input")
                .long("input")
                .required(false)
                .help("Optional input file. If not provided, defaults to ./files/<challenge>"),
        )
        .get_matches();

    let challenge = matches
        .get_one::<String>("challenge")
        .expect("You must provide a valid challenge number.\nCorrect usage: ./aoc2023 <challenge>");
    let challenge = challenge.parse::<u8>().expect(&format!(
        "Challenge number must be a valid number.\nProvided challenge: {}",
        challenge
    ));
    let default_input_path = format!("./inputs/{}.aoc", challenge);
    let input = matches
        .get_one::<String>("input")
        .unwrap_or(&default_input_path);

    let path = Path::new(input);
    let file = File::open(&path).expect(&format!(
        "Could not find input file at specified path.\nProvided path: {}",
        path.display()
    ));

    let solution = ChallengeFactory::create(&challenge, &file);

    println!("Part One: {}", solution.solve_part_one());
    println!("Part Two: {}", solution.solve_part_two());
}
