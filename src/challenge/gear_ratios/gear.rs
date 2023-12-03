use std::collections::HashSet;

use super::part_number::PartNumber;

pub struct Gear {
    part_numbers: HashSet<PartNumber>,
}

impl Gear {
    pub fn new() -> Self {
        Gear {
            part_numbers: HashSet::new(),
        }
    }

    pub fn add_part(&mut self, part_number: &PartNumber) {
        if !self.part_numbers.contains(part_number) {
            self.part_numbers.insert(part_number.clone());
        }
    }

    pub fn get_ratio(&self) -> u32 {
        self.part_numbers.iter().map(|part| part.value).product()
    }

    pub fn is_valid(&self) -> bool {
        self.part_numbers.len() == 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ch03_gear_add_part_no_duplicates() {
        let mut gear = Gear::new();
        let part = PartNumber {
            row: 2,
            col: 3,
            length: 2,
            value: 73,
        };

        gear.add_part(&part.clone());
        gear.add_part(&part.clone());

        assert_eq!(gear.part_numbers.len(), 1);
    }

    #[test]
    fn ch03_gear_get_ratio() {
        let mut gear = Gear::new();
        let part1 = PartNumber {
            row: 2,
            col: 3,
            length: 2,
            value: 73,
        };

        let part2 = PartNumber {
            row: 4,
            col: 1,
            length: 2,
            value: 10,
        };

        gear.add_part(&part1.clone());
        gear.add_part(&part2.clone());

        assert_eq!(gear.get_ratio(), 730);
    }
}
