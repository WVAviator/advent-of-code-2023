use crate::utils::to_lines_vec;

use self::map::Map;

use super::Challenge;
use std::fs::File;

mod map;
mod node;

#[derive(Default)]
pub struct HauntedWasteland {
    lines: Vec<String>,
}

impl Challenge for HauntedWasteland {
    fn load(&mut self, file: &File) {
        self.lines = to_lines_vec(file);
    }
    fn solve_part_one(&self) -> String {
        let mut lines_iter = self.lines.iter();
        let directions = lines_iter.next().expect("Could not parse directions.");
        let mut map = Map::new(lines_iter.skip(1).map(|line| line.clone()).collect());

        let mut path = directions.chars().cycle();
        let mut steps = 1;
        while map.travel(&path.next().expect("Path ended?")) != "ZZZ" {
            steps += 1;
        }

        format!("{}", steps)
    }
    fn solve_part_two(&self) -> String {
        format!("Not implemented yet!")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_lines() -> Vec<String> {
        vec![
            String::from("RL"),
            String::from(""),
            String::from("AAA = (BBB, CCC)"),
            String::from("BBB = (DDD, EEE)"),
            String::from("CCC = (ZZZ, GGG)"),
            String::from("DDD = (DDD, DDD)"),
            String::from("EEE = (EEE, EEE)"),
            String::from("GGG = (GGG, GGG)"),
            String::from("ZZZ = (ZZZ, ZZZ)"),
        ]
    }

    #[test]
    fn ch08_haunted_wasteland_part_one() {
        let lines = get_lines();
        let mut haunted_wasteland = HauntedWasteland::default();
        haunted_wasteland.lines = lines;

        assert_eq!(haunted_wasteland.solve_part_one(), "2");
    }
}
