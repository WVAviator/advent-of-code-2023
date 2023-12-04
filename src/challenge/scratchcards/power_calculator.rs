use super::score_calculator::ScoreCalculator;

pub struct PowerCalculator {}

impl PowerCalculator {
    pub fn new() -> Self {
        PowerCalculator {}
    }
}

impl ScoreCalculator for PowerCalculator {
    fn calculate_card(scratchcard: &super::scratchcard::Scratchcard) -> u32 {
        let count = scratchcard.get_matches().len() as u32;
        if count == 0 {
            return 0;
        }
        let base: u32 = 2;
        return base.pow(count - 1);
    }

    fn calculate_total(&mut self, scratchcards: &Vec<super::scratchcard::Scratchcard>) -> u32 {
        scratchcards
            .iter()
            .map(|card| PowerCalculator::calculate_card(card))
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::challenge::scratchcards::scratchcard::Scratchcard;

    use super::*;

    #[test]
    fn ch04_power_calculator_calculate_card() {
        let card = Scratchcard::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let score = PowerCalculator::calculate_card(&card);
        assert_eq!(score, 8);
    }

    #[test]
    fn ch04_power_calculator_calculate_total() {
        let card1 = Scratchcard::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let card2 = Scratchcard::parse("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
        let card3 = Scratchcard::parse("Card 3: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");

        let score = PowerCalculator::new().calculate_total(&vec![card1, card2, card3]);
        assert_eq!(score, 10);
    }
}
