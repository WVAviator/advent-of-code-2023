use crate::utils::to_lines_vec;

use self::engine_schematic::EngineSchematic;

use super::Challenge;
use std::fs::File;

mod adjacent;
mod engine_schematic;
mod gear;
mod part_number;

#[derive(Default)]
pub struct GearRatios {
    engine_schematic: EngineSchematic,
}

impl Challenge for GearRatios {
    fn load(&mut self, file: &File) {
        self.engine_schematic = EngineSchematic::new(to_lines_vec(file));
    }
    fn solve_part_one(&self) -> String {
        let total: u32 = self
            .engine_schematic
            .get_nums_adjacent_to_symbols()
            .iter()
            .map(|part_number| part_number.value)
            .sum();

        format!("{}", total)
    }
    fn solve_part_two(&self) -> String {
        let total: u32 = self
            .engine_schematic
            .get_gear_ratios_adjacent_to_two_numbers()
            .iter()
            .sum();

        format!("{}", total)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            String::from("467..114.."),
            String::from("...*......"),
            String::from("..35..633."),
            String::from("......#..."),
            String::from("617*......"),
            String::from(".....+.58."),
            String::from("..592....."),
            String::from("......755."),
            String::from("...$.*...."),
            String::from(".664.598.."),
        ]
    }

    #[test]
    fn ch03_part_one() {
        let input = get_test_input();
        let mut gear_ratios = GearRatios::default();
        gear_ratios.engine_schematic = EngineSchematic::new(input);

        assert_eq!(gear_ratios.solve_part_one(), "4361");
    }

    #[test]
    fn ch03_part_two() {
        let input = get_test_input();
        let mut gear_ratios = GearRatios::default();
        gear_ratios.engine_schematic = EngineSchematic::new(input);

        assert_eq!(gear_ratios.solve_part_two(), "467835");
    }
}
