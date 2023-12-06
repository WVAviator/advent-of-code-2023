use crate::utils::to_lines_vec;

use self::{almanac::Almanac, resource::Resource};

use super::Challenge;
use std::fs::File;

mod almanac;
mod resource;
mod resource_map;

#[derive(Default)]
pub struct IfYouGiveASeedAFertilizer {
    seeds: Vec<num::BigInt>,
    almanac: Almanac,
}

impl Challenge for IfYouGiveASeedAFertilizer {
    fn load(&mut self, file: &File) {
        let lines = to_lines_vec(file);
        self.load_lines(lines);
    }
    fn solve_part_one(&self) -> String {
        let min_location = self
            .seeds
            .iter()
            .map(|seed| {
                let (value, resource) = self.almanac.map_through(seed.clone(), Resource::Seed);
                if resource != Resource::Location {
                    panic!("Could not map seed to location.");
                }
                value
            })
            .min()
            .expect("No elements in location calculation.");

        format!("{}", min_location)
    }
    fn solve_part_two(&self) -> String {
        let resource_map = self.almanac.condense();
        let min_location = self
            .seeds
            .chunks(2)
            .map(|range| resource_map.lowest_overlap(&range[0], &range[1]))
            .min()
            .expect("Could not map through seed value.");

        format!("{}", min_location)
    }
}

impl IfYouGiveASeedAFertilizer {
    fn load_lines(&mut self, lines: Vec<String>) {
        self.seeds = lines[0]
            .split(' ')
            .skip(1)
            .map(|num_str| num_str.parse().expect("Could not parse seed into number."))
            .collect();
        self.almanac = Almanac::new(lines.into_iter().skip(2).collect());
    }
}
#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> IfYouGiveASeedAFertilizer {
        let lines = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ]
        .iter()
        .map(|line| line.to_string())
        .collect();

        let mut seed_fertilizer = IfYouGiveASeedAFertilizer::default();
        seed_fertilizer.load_lines(lines);
        seed_fertilizer
    }

    #[test]
    fn ch05_seed_fertilizer_part_one() {
        let seed_fertilizer = get_test_input();
        assert_eq!(seed_fertilizer.solve_part_one(), "35");
    }

    #[test]
    fn cho5_seed_fertilizer_part_two() {
        let seed_fertilizer = get_test_input();
        assert_eq!(seed_fertilizer.solve_part_two(), "46");
    }
}
