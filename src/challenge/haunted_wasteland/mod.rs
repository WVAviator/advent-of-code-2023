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
        let map = Map::new(lines_iter.skip(1).map(|line| line.clone()).collect());

        let mut path = directions.chars().cycle();
        let mut steps = 1;
        let mut location = "AAA";
        loop {
            location = map.travel(location, &path.next().expect("Path ended?"));
            if location == "ZZZ" {
                break;
            }
            steps += 1;
        }

        format!("{}", steps)
    }
    fn solve_part_two(&self) -> String {
        let mut lines_iter = self.lines.iter();
        let directions = lines_iter.next().expect("Could not parse directions.");
        let map = Map::new(lines_iter.skip(1).map(|line| line.clone()).collect());

        let mut path = directions.chars().cycle();
        let mut locations: Vec<&str> = map
            .iter()
            .filter(|(_, v)| v.matches("__A"))
            .map(|(_, v)| v.address.as_str())
            .collect();
        let mut steps = 1;

        loop {
            if steps % 1000000 == 0 {
                println!("Travelled {} steps.", steps);
            }
            let next_path = path.next().expect("Path ended?");
            locations = locations
                .into_iter()
                .map(|location| map.travel(location, &next_path))
                .collect();
            if locations
                .iter()
                .all(|location| location.chars().nth(2) == Some('Z'))
            {
                break;
            }
            steps += 1;
        }

        format!("{}", steps)
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

    #[test]
    fn ch08_haunted_wasteland_part_two() {
        let lines = vec![
            String::from("LR"),
            String::from(""),
            String::from("11A = (11B, XXX)"),
            String::from("11B = (XXX, 11Z)"),
            String::from("11Z = (11B, XXX)"),
            String::from("22A = (22B, XXX)"),
            String::from("22B = (22C, 22C)"),
            String::from("22C = (22Z, 22Z)"),
            String::from("22Z = (22B, 22B)"),
            String::from("XXX = (XXX, XXX)"),
        ];
        let mut haunted_wasteland = HauntedWasteland::default();
        haunted_wasteland.lines = lines;

        assert_eq!(haunted_wasteland.solve_part_two(), "6");
    }
}
