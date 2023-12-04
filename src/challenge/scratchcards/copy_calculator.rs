use std::collections::HashMap;

use super::{score_calculator::ScoreCalculator, scratchcard::Scratchcard};

pub struct CopyCalculator {
    copies: HashMap<u32, u32>,
}

impl CopyCalculator {
    pub fn new() -> Self {
        let copies = HashMap::new();
        CopyCalculator { copies }
    }
}

impl ScoreCalculator for CopyCalculator {
    fn calculate_card(scratchcard: &super::scratchcard::Scratchcard) -> u32 {
        let count = scratchcard.get_matches().len() as u32;
        count
    }

    fn calculate_total(&mut self, cards: &Vec<Scratchcard>) -> u32 {
        cards.iter().for_each(|card| {
            *self.copies.entry(card.id).or_insert(0) += 1;
            let card_copies = self.copies.get(&card.id).unwrap().clone();
            let score = CopyCalculator::calculate_card(card);

            for i in 1..=score {
                *self.copies.entry(card.id + i).or_insert(0) += card_copies;
            }
        });

        self.copies.iter().map(|(_, copy_count)| copy_count).sum()
    }
}

#[cfg(test)]
mod test {
    use crate::challenge::scratchcards::scratchcard::Scratchcard;

    use super::*;

    #[test]
    fn ch04_copy_calculator_calculate_card() {
        let card = Scratchcard::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let score = CopyCalculator::calculate_card(&card);
        assert_eq!(score, 4);
    }

    #[test]
    fn ch04_copy_calculator_calculate_total() {
        let card1 = Scratchcard::parse("Card 1: 10 11 12 13 14 | 10 11 17 18 19 20 21 22");
        let card2 = Scratchcard::parse("Card 2: 10 11 12 13 14 | 10 16 17 18 19 20 21 22");
        let card3 = Scratchcard::parse("Card 3: 10 11 12 13 14 | 15 16 17 18 19 20 21 22");

        let score = CopyCalculator::new().calculate_total(&vec![card1, card2, card3]);

        assert_eq!(score, 7);
    }
}
