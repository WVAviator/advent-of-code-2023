use crate::utils::to_lines_vec;

use self::{
    copy_calculator::CopyCalculator, power_calculator::PowerCalculator,
    score_calculator::ScoreCalculator, scratchcard::Scratchcard,
};

use super::Challenge;
use std::fs::File;

mod copy_calculator;
mod power_calculator;
mod score_calculator;
mod scratchcard;

#[derive(Default)]
pub struct Scratchcards {
    scratchcards: Vec<Scratchcard>,
}

impl Challenge for Scratchcards {
    fn load(&mut self, file: &File) {
        self.scratchcards = to_lines_vec(file)
            .into_iter()
            .map(|line| Scratchcard::parse(&line))
            .collect();
    }
    fn solve_part_one(&self) -> String {
        let total = PowerCalculator::new().calculate_total(&self.scratchcards);

        format!("{}", total)
    }
    fn solve_part_two(&self) -> String {
        let total = CopyCalculator::new().calculate_total(&self.scratchcards);

        format!("{}", total)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_test_case() -> Scratchcards {
        let lines = vec![
            String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
        ];
        Scratchcards {
            scratchcards: lines
                .into_iter()
                .map(|line| Scratchcard::parse(&line))
                .collect(),
        }
    }

    #[test]
    fn ch04_scratchcards_part_one() {
        let scratchcards = create_test_case();
        assert_eq!(scratchcards.solve_part_one(), "13");
    }

    #[test]
    fn ch04_scratchcards_part_two() {
        let scratchcards = create_test_case();
        assert_eq!(scratchcards.solve_part_two(), "30");
    }
}
