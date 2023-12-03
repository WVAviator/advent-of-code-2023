use super::{adjacent::Adjacent, part_number::PartNumber};

#[derive(Default)]
pub struct EngineSchematic {
    schematic: Vec<Vec<char>>,
    part_numbers: Vec<PartNumber>,
}

impl EngineSchematic {
    pub fn new(lines: Vec<String>) -> Self {
        let mut schematic: Vec<Vec<char>> = Vec::new();
        lines.into_iter().for_each(|line| {
            let chars = line.chars().collect();
            schematic.push(chars);
        });

        let mut part_numbers = Vec::new();

        for (row_index, row) in schematic.iter().enumerate() {
            let mut col_index = 0;
            while col_index < row.len() {
                if let Ok(part_number) = PartNumber::new(row_index, &row, col_index) {
                    part_numbers.push(part_number);
                    col_index += part_number.length;
                    continue;
                }
                col_index += 1;
            }
        }

        EngineSchematic {
            schematic,
            part_numbers,
        }
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

    fn symbol_adjacent(&self, row: usize, col: usize) -> bool {
        Adjacent::new(&self.schematic, (row, col)).any(|ch| EngineSchematic::is_symbol(ch))
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
    fn ch03_is_symbol() {
        assert!(EngineSchematic::is_symbol('*'));
        assert!(EngineSchematic::is_symbol('%'));
        assert!(!EngineSchematic::is_symbol('3'));
        assert!(!EngineSchematic::is_symbol('.'));
    }

    #[test]
    fn ch03_symbol_adjacent() {
        let schematic = get_test_schematic();

        assert!(schematic.symbol_adjacent(1, 1));
        assert!(schematic.symbol_adjacent(2, 1));
        assert!(schematic.symbol_adjacent(2, 4));
        assert!(!schematic.symbol_adjacent(0, 0));
        assert!(!schematic.symbol_adjacent(0, 2));
    }

    #[test]
    fn ch03_get_nums_adjacent_to_symbols_all() {
        let schematic = get_test_schematic();

        let expected = schematic.part_numbers.clone();
        assert_eq!(expected, schematic.get_nums_adjacent_to_symbols());
    }

    #[test]
    fn ch03_get_nums_adjacent_to_symbols_some() {
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
}
