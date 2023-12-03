use std::collections::HashMap;

use super::{adjacent::Adjacent, gear::Gear, part_number::PartNumber};

#[derive(Default)]
pub struct EngineSchematic {
    schematic: Vec<Vec<char>>,
    part_numbers: Vec<PartNumber>,
    gears: HashMap<(usize, usize), Gear>,
}

impl EngineSchematic {
    pub fn new(lines: Vec<String>) -> Self {
        let mut schematic: Vec<Vec<char>> = Vec::new();
        lines.into_iter().for_each(|line| {
            let chars = line.chars().collect();
            schematic.push(chars);
        });

        let mut part_numbers = Vec::new();
        let mut gears = HashMap::new();

        for (row_index, row) in schematic.iter().enumerate() {
            let mut col_index = 0;
            while col_index < row.len() {
                if row.get(col_index).unwrap().eq(&'*') {
                    let gear = Gear::new();
                    gears.insert((row_index, col_index), gear);
                }
                if let Ok(part_number) = PartNumber::new(row_index, &row, col_index) {
                    part_numbers.push(part_number);
                    col_index += part_number.length;
                    continue;
                }
                col_index += 1;
            }
        }

        let mut schematic = EngineSchematic {
            schematic,
            part_numbers,
            gears,
        };

        schematic.connect_all_gears();

        schematic
    }

    pub fn get_nums_adjacent_to_symbols(&self) -> Vec<PartNumber> {
        self.part_numbers
            .clone()
            .iter()
            .filter(|part_number| {
                for i in part_number.col..(part_number.col + part_number.length) {
                    if self.symbol_adjacent(part_number.row, i) {
                        return true;
                    }
                }
                return false;
            })
            .map(|p| p.clone())
            .collect()
    }

    pub fn get_gear_ratios_adjacent_to_two_numbers(&self) -> Vec<u32> {
        self.gears
            .iter()
            .filter(|(_, gear)| gear.is_valid())
            .map(|(_, gear)| gear.get_ratio())
            .collect()
    }

    fn connect_all_gears(&mut self) {
        self.part_numbers
            .clone()
            .iter()
            .for_each(|part_number| self.connect_gears(part_number));
    }

    fn symbol_adjacent(&self, row: usize, col: usize) -> bool {
        Adjacent::new(&self.schematic, (row, col)).any(|(ch, _, _)| EngineSchematic::is_symbol(ch))
    }

    fn connect_gears(&mut self, part_number: &PartNumber) {
        for i in part_number.col..(part_number.col + part_number.length) {
            Adjacent::new(&self.schematic, (part_number.row, i)).for_each(|(ch, r, c)| {
                if ch.eq(&'*') {
                    let gear: &mut Gear =
                        self.gears.get_mut(&(r, c)).expect("Could not find gear.");
                    gear.add_part(&part_number);
                }
            });
        }
    }

    fn is_symbol(val: char) -> bool {
        if val.eq(&'.') {
            return false;
        }

        match val.to_digit(10) {
            Some(_) => false,
            None => true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_schematic() -> EngineSchematic {
        let input = vec![
            String::from("..123"),
            String::from("12..$"),
            String::from("..+.."),
        ];
        let engine_schematic = EngineSchematic::new(input);

        engine_schematic
    }

    #[test]
    fn ch03_engine_schematic_new_schematic_correct() {
        let engine_schematic = get_test_schematic();

        let expected = vec![
            vec!['.', '.', '1', '2', '3'],
            vec!['1', '2', '.', '.', '$'],
            vec!['.', '.', '+', '.', '.'],
        ];

        assert_eq!(expected, engine_schematic.schematic);
    }

    #[test]
    fn ch03_engine_schematic_new_part_numbers_correct() {
        let engine_schematic = get_test_schematic();

        let expected = vec![
            PartNumber {
                row: 0,
                col: 2,
                length: 3,
                value: 123,
            },
            PartNumber {
                row: 1,
                col: 0,
                length: 2,
                value: 12,
            },
        ];

        assert_eq!(expected, engine_schematic.part_numbers);
    }

    #[test]
    fn ch03_engine_schematic_is_symbol() {
        assert!(EngineSchematic::is_symbol('*'));
        assert!(EngineSchematic::is_symbol('%'));
        assert!(!EngineSchematic::is_symbol('3'));
        assert!(!EngineSchematic::is_symbol('.'));
    }

    #[test]
    fn ch03_engine_schematic_symbol_adjacent() {
        let schematic = get_test_schematic();

        assert!(schematic.symbol_adjacent(1, 1));
        assert!(schematic.symbol_adjacent(2, 1));
        assert!(schematic.symbol_adjacent(2, 4));
        assert!(!schematic.symbol_adjacent(0, 0));
        assert!(!schematic.symbol_adjacent(0, 2));
    }

    #[test]
    fn ch03_engine_schematic_get_nums_adjacent_to_symbols_all() {
        let schematic = get_test_schematic();

        let expected = schematic.part_numbers.clone();
        assert_eq!(expected, schematic.get_nums_adjacent_to_symbols());
    }

    #[test]
    fn ch03_engine_schematic_get_nums_adjacent_to_symbols_some() {
        let input = vec![
            String::from("..123"),
            String::from("12..."),
            String::from("..+.."),
        ];
        let schematic = EngineSchematic::new(input);

        let expected = vec![PartNumber {
            row: 1,
            col: 0,
            length: 2,
            value: 12,
        }];

        assert_eq!(expected, schematic.get_nums_adjacent_to_symbols());
    }

    #[test]
    fn ch03_engine_schematic_connect_gears() {
        let input = vec![
            String::from("..123"),
            String::from("10*.."),
            String::from("....."),
        ];
        let mut schematic = EngineSchematic::new(input);
        for part in schematic.part_numbers.clone() {
            schematic.connect_gears(&part);
        }
        assert_eq!(schematic.gears.len(), 1);
        schematic.gears.iter().for_each(|(_, gear)| {
            assert_eq!(gear.get_ratio(), 1230);
        });
    }

    #[test]
    fn ch03_engine_schematic_get_gear_ratios_adjacent_to_two_numbers() {
        let input = vec![
            String::from("..123"),
            String::from("10*.."),
            String::from("...*2"),
        ];
        let schematic = EngineSchematic::new(input);
        let ratios = schematic.get_gear_ratios_adjacent_to_two_numbers();
        assert_eq!(ratios, vec![1230]);
    }
}
