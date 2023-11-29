use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn to_lines_vec(file: &File) -> Vec<String> {
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect::<Vec<String>>()
}
